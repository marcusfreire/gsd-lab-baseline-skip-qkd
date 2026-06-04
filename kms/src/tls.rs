use std::{
    io::{self, BufReader, Cursor},
    sync::Arc,
};

use anyhow::{Context, anyhow};
use axum::Extension;
use axum_server::{
    accept::Accept,
    tls_rustls::{RustlsAcceptor, RustlsConfig},
};
use futures_util::future::BoxFuture;
use rustls::{
    RootCertStore, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::server::TlsStream;
use tracing::info;
use tower_layer::Layer;
use x509_parser::{certificate::X509Certificate, prelude::FromDer};

use crate::config::TlsConfig as AppTlsConfig;

#[derive(Clone, Debug)]
pub struct AuthenticatedPeer {
    pub sae_id: String,
    pub cert_subject: String,
}

#[derive(Clone)]
pub struct AuthenticatedPeerAcceptor {
    inner: RustlsAcceptor,
}

impl AuthenticatedPeerAcceptor {
    pub fn new(config: RustlsConfig) -> Self {
        Self {
            inner: RustlsAcceptor::new(config),
        }
    }
}

impl<I, S> Accept<I, S> for AuthenticatedPeerAcceptor
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: Send + 'static,
{
    type Stream = TlsStream<I>;
    type Service = <Extension<AuthenticatedPeer> as tower_layer::Layer<S>>::Service;
    type Future = BoxFuture<'static, io::Result<(Self::Stream, Self::Service)>>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        let acceptor = self.inner.clone();
        Box::pin(async move {
            let (stream, service) = acceptor.accept(stream, service).await?;
            let authenticated_peer = extract_authenticated_peer(&stream)
                .map_err(|err| io::Error::new(io::ErrorKind::PermissionDenied, err))?;
            let service = Extension(authenticated_peer).layer(service);
            Ok((stream, service))
        })
    }
}

pub fn build_rustls_config(config: &AppTlsConfig) -> anyhow::Result<RustlsConfig> {
    let server_certs = load_certificates(&config.server_cert_path)
        .with_context(|| format!("failed to load server certs from {}", config.server_cert_path))?;
    let server_key = load_private_key(&config.server_key_path)
        .with_context(|| format!("failed to load server key from {}", config.server_key_path))?;
    let client_roots = load_root_store(&config.client_ca_cert_path).with_context(|| {
        format!(
            "failed to load trusted client CAs from {}",
            config.client_ca_cert_path
        )
    })?;

    let client_verifier = WebPkiClientVerifier::builder(Arc::new(client_roots))
        .build()
        .context("failed to build client certificate verifier")?;

    let mut server_config = ServerConfig::builder()
        .with_client_cert_verifier(client_verifier)
        .with_single_cert(server_certs, server_key)
        .context("failed to build rustls server config")?;
    server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

    info!(
        server_cert_path = %config.server_cert_path,
        client_ca_cert_path = %config.client_ca_cert_path,
        "configured rustls with required client authentication"
    );

    Ok(RustlsConfig::from_config(Arc::new(server_config)))
}

fn load_certificates(path: &str) -> anyhow::Result<Vec<CertificateDer<'static>>> {
    let pem = std::fs::read(path)?;
    rustls_pemfile::certs(&mut BufReader::new(Cursor::new(pem)))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| anyhow!(err))
}

fn load_private_key(path: &str) -> anyhow::Result<PrivateKeyDer<'static>> {
    let pem = std::fs::read(path)?;
    let key = rustls_pemfile::private_key(&mut BufReader::new(Cursor::new(pem)))
        .map_err(|err| anyhow!(err))?
        .ok_or_else(|| anyhow!("no private key found in PEM file"))?;
    Ok(key)
}

fn load_root_store(path: &str) -> anyhow::Result<RootCertStore> {
    let certs = load_certificates(path)?;
    let mut store = RootCertStore::empty();
    for cert in certs {
        store
            .add(cert)
            .map_err(|err| anyhow!("invalid CA certificate: {err}"))?;
    }
    Ok(store)
}

fn extract_authenticated_peer<I>(stream: &TlsStream<I>) -> anyhow::Result<AuthenticatedPeer> {
    let (_, connection) = stream.get_ref();
    let client_certs = connection
        .peer_certificates()
        .ok_or_else(|| anyhow!("client certificate is missing"))?;
    let client_cert = client_certs
        .first()
        .ok_or_else(|| anyhow!("client certificate chain is empty"))?;

    parse_authenticated_peer(client_cert)
}

fn parse_authenticated_peer(client_cert: &CertificateDer<'_>) -> anyhow::Result<AuthenticatedPeer> {
    let (_, cert) =
        X509Certificate::from_der(client_cert.as_ref()).map_err(|err| anyhow!(err))?;
    let sae_id = extract_subject_common_name(&cert)
        .ok_or_else(|| anyhow!("client certificate subject CN is missing"))?;

    Ok(AuthenticatedPeer {
        sae_id,
        cert_subject: cert.subject().to_string(),
    })
}

fn extract_subject_common_name(cert: &X509Certificate<'_>) -> Option<String> {
    cert.subject()
        .iter_common_name()
        .find_map(|cn| cn.as_str().ok().map(ToOwned::to_owned))
}
