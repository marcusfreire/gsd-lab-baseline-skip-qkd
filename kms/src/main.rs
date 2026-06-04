use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Context;
use clap::Parser;
use config::Config;
use tracing::{debug, info};

mod config;
mod routes;
mod storage;
#[cfg(test)]
mod tests;
mod tls;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Path for the configuration file
    #[arg(short, long)]
    config: PathBuf,
}

struct AppState {
    storage: Box<dyn storage::Etsi014KeyStorage>,
    config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = Config::load(&args.config)?;

    debug!("Loaded config from {:?}: {:#?}", args.config, config);

    let storage = storage::create_storage_from_config(&config.storage).await?;

    let app_state = Arc::new(AppState { storage, config });

    let app = routes::build_router(Arc::clone(&app_state));

    if app_state.config.use_mtls {
        let tls_config = app_state
            .config
            .tls
            .as_ref()
            .context("mTLS is enabled but no tls config section was provided")?;
        let rustls_config = tls::build_rustls_config(tls_config)?;
        let acceptor = tls::AuthenticatedPeerAcceptor::new(rustls_config);
        let listen_addr: SocketAddr = app_state
            .config
            .listen
            .parse()
            .context("listen must be a socket address when use_mTLS is enabled")?;

        info!(listen = %listen_addr, "starting HTTPS server with required client authentication");
        axum_server::bind(listen_addr)
            .acceptor(acceptor)
            .serve(app.into_make_service())
            .await?;
    } else {
        let listener = tokio::net::TcpListener::bind(&app_state.config.listen).await?;
        info!(listen = %app_state.config.listen, "starting HTTP server without mTLS");
        axum::serve(listener, app).await?;
    }

    Ok(())
}
