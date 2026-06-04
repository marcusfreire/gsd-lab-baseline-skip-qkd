use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    Extension,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use getrandom::getrandom;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{AppState, config::Config, storage::KeyData, tls::AuthenticatedPeer};

const SAE_ID_HEADER: &str = "x-sae-id";
const DEFAULT_KEY_SIZE_BITS: u32 = 256;
const DEFAULT_MAX_KEYS_PER_REQUEST: u32 = 128;
const DEFAULT_MAX_KEY_COUNT: u32 = 1_000_000;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new()
        // Report key delivery status for a master SAE requesting keys for the slave SAE {slave_sae_id}.
        .route("/{slave_sae_id}/status", get(get_status))
        .route(
            // Generate fresh encryption keys for the slave SAE {slave_sae_id} on behalf of the calling master SAE.
            "/{slave_sae_id}/enc_keys",
            get(get_enc_keys).post(post_enc_keys),
        )
        .route(
            // Return previously generated decryption keys for the calling slave SAE using the original master SAE {master_sae_id}.
            "/{master_sae_id}/dec_keys",
            get(get_dec_keys).post(post_dec_keys),
        )
}

#[derive(Debug, Serialize)]
struct StatusResponse {
    #[serde(rename = "source_KME_ID")]
    source_kme_id: String,
    #[serde(rename = "target_KME_ID")]
    target_kme_id: String,
    #[serde(rename = "master_SAE_ID")]
    master_sae_id: String,
    #[serde(rename = "slave_SAE_ID")]
    slave_sae_id: String,
    key_size: u32,
    stored_key_count: u32,
    max_key_count: u32,
    max_key_per_request: u32,
    max_key_size: u32,
    min_key_size: u32,
    #[serde(rename = "max_SAE_ID_count")]
    max_sae_id_count: u32,
    status_extension: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Default, Deserialize)]
struct GetKeyRequest {
    number: Option<u32>,
    size: Option<u32>,
    #[serde(rename = "additional_slave_SAE_IDs", default)]
    additional_slave_sae_ids: Vec<String>,
    #[serde(rename = "extension_mandatory", default)]
    extension_mandatory: Vec<HashMap<String, serde_json::Value>>,
    #[serde(rename = "extension_optional", default)]
    extension_optional: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
struct GetEncKeysQuery {
    number: Option<u32>,
    size: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct GetDecKeysQuery {
    #[serde(rename = "key_ID")]
    key_id: Uuid,
}

#[derive(Debug, Serialize)]
struct KeyContainerResponse {
    keys: Vec<KeyResponse>,
    key_container_extension: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct KeyResponse {
    #[serde(rename = "key_ID")]
    key_id: Uuid,
    key: String,
}

#[derive(Debug, Deserialize)]
struct GetKeyIdsRequest {
    #[serde(rename = "key_IDs")]
    key_ids: Vec<KeyIdRequest>,
}

#[derive(Debug, Deserialize)]
struct KeyIdRequest {
    #[serde(rename = "key_ID")]
    key_id: Uuid,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
struct ApiError {
    status: StatusCode,
    message: Option<String>,
}

impl ApiError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: Some(message.into()),
        }
    }

    fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: None,
        }
    }

    fn service_unavailable(err: anyhow::Error) -> Self {
        error!(error = ?err, "backend error");
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            message: Some("server error".to_string()),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self.message {
            Some(message) => (self.status, Json(ErrorResponse { message })).into_response(),
            None => self.status.into_response(),
        }
    }
}

/// Returns delivery status for the authenticated master SAE and the target slave SAE in the path.
async fn get_status(
    State(app_state): State<Arc<AppState>>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    Path(slave_sae_id): Path<String>,
) -> Result<Json<StatusResponse>, ApiError> {
    let master_sae_id =
        resolve_calling_sae_id(&app_state, &headers, authenticated_peer.as_ref().map(|peer| &peer.0))?;
    let source_kme_id = find_kme_for_sae(&app_state.config, &master_sae_id)
        .ok_or_else(|| ApiError::unauthorized())?
        .to_string();
    let target_kme_id = find_kme_for_sae(&app_state.config, &slave_sae_id)
        .ok_or_else(|| ApiError::bad_request("unknown slave SAE ID"))?
        .to_string();

    info!(
        master_sae_id,
        slave_sae_id,
        source_kme_id,
        target_kme_id,
        "status requested"
    );

    Ok(Json(StatusResponse {
        source_kme_id,
        target_kme_id,
        master_sae_id,
        slave_sae_id,
        key_size: DEFAULT_KEY_SIZE_BITS,
        stored_key_count: DEFAULT_MAX_KEY_COUNT,
        max_key_count: DEFAULT_MAX_KEY_COUNT,
        max_key_per_request: DEFAULT_MAX_KEYS_PER_REQUEST,
        max_key_size: DEFAULT_KEY_SIZE_BITS,
        min_key_size: DEFAULT_KEY_SIZE_BITS,
        max_sae_id_count: 0,
        status_extension: HashMap::new(),
    }))
}

/// Handles the ETSI014 GET shorthand for enc_keys, where only `number` and `size` may be provided as query parameters.
async fn get_enc_keys(
    State(app_state): State<Arc<AppState>>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    Path(slave_sae_id): Path<String>,
    Query(query): Query<GetEncKeysQuery>,
) -> Result<Json<KeyContainerResponse>, ApiError> {
    let request = GetKeyRequest {
        number: query.number,
        size: query.size,
        ..Default::default()
    };

    issue_keys(app_state, authenticated_peer, headers, slave_sae_id, request).await
}

/// Handles the full ETSI014 POST form for enc_keys and issues fresh keys for the requested slave SAE.
async fn post_enc_keys(
    State(app_state): State<Arc<AppState>>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    Path(slave_sae_id): Path<String>,
    maybe_body: Option<Json<GetKeyRequest>>,
) -> Result<Json<KeyContainerResponse>, ApiError> {
    let request = maybe_body.map(|body| body.0).unwrap_or_default();
    issue_keys(app_state, authenticated_peer, headers, slave_sae_id, request).await
}

/// Generates, stores, and returns one or more fresh keys bound to the authenticated master SAE and the requested slave SAE.
async fn issue_keys(
    app_state: Arc<AppState>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    slave_sae_id: String,
    request: GetKeyRequest,
) -> Result<Json<KeyContainerResponse>, ApiError> {
    let master_sae_id =
        resolve_calling_sae_id(&app_state, &headers, authenticated_peer.as_ref().map(|peer| &peer.0))?;

    validate_slave_sae_id(&app_state.config, &slave_sae_id)?;
    validate_get_key_request(&request)?;

    let number = request.number.unwrap_or(1);

    info!(master_sae_id, slave_sae_id, number, "issuing keys");

    let mut keys = Vec::with_capacity(number as usize);

    for _ in 0..number {
        let key_id = Uuid::new_v4();
        let key_bytes = generate_key_material();
        let key_data = KeyData {
            master_sae_id: master_sae_id.clone(),
            slave_sae_id: slave_sae_id.clone(),
            key: key_bytes,
        };

        app_state
            .storage
            .register_or_update_key(key_id, &key_data)
            .await
            .map_err(ApiError::service_unavailable)?;

        keys.push(KeyResponse {
            key_id,
            key: STANDARD.encode(key_data.key),
        });
    }

    Ok(Json(KeyContainerResponse {
        keys,
        key_container_extension: HashMap::new(),
    }))
}

/// Handles the ETSI014 GET shorthand for dec_keys, where a single `key_ID` is supplied as a query parameter.
async fn get_dec_keys(
    State(app_state): State<Arc<AppState>>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    Path(master_sae_id): Path<String>,
    Query(query): Query<GetDecKeysQuery>,
) -> Result<Json<KeyContainerResponse>, ApiError> {
    retrieve_keys(
        app_state,
        authenticated_peer,
        headers,
        master_sae_id,
        GetKeyIdsRequest {
            key_ids: vec![KeyIdRequest {
                key_id: query.key_id,
            }],
        },
    )
    .await
}

/// Handles the full ETSI014 POST form for dec_keys and retrieves keys for the authenticated slave SAE.
async fn post_dec_keys(
    State(app_state): State<Arc<AppState>>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    Path(master_sae_id): Path<String>,
    Json(request): Json<GetKeyIdsRequest>,
) -> Result<Json<KeyContainerResponse>, ApiError> {
    retrieve_keys(app_state, authenticated_peer, headers, master_sae_id, request).await
}

/// Loads keys by ID, verifies they belong to the authenticated slave SAE and the path master SAE, then consumes them.
async fn retrieve_keys(
    app_state: Arc<AppState>,
    authenticated_peer: Option<Extension<AuthenticatedPeer>>,
    headers: HeaderMap,
    master_sae_id: String,
    request: GetKeyIdsRequest,
) -> Result<Json<KeyContainerResponse>, ApiError> {
    let slave_sae_id =
        resolve_calling_sae_id(&app_state, &headers, authenticated_peer.as_ref().map(|peer| &peer.0))?;

    if find_kme_for_sae(&app_state.config, &master_sae_id).is_none() {
        return Err(ApiError::bad_request("unknown master SAE ID"));
    }

    if request.key_ids.is_empty() {
        return Err(ApiError::bad_request("at least one key_ID is required"));
    }

    info!(
        master_sae_id,
        slave_sae_id,
        key_count = request.key_ids.len(),
        "retrieving keys"
    );

    let mut loaded_keys = Vec::with_capacity(request.key_ids.len());

    for key_request in request.key_ids {
        let key_data = app_state
            .storage
            .get_key_by_id(key_request.key_id)
            .await
            .map_err(ApiError::service_unavailable)?
            .ok_or_else(|| {
                ApiError::bad_request("one or more keys specified are not found on KME")
            })?;

        if key_data.slave_sae_id != slave_sae_id {
            warn!(
                requested_key_id = %key_request.key_id,
                expected_slave_sae_id = key_data.slave_sae_id,
                actual_slave_sae_id = slave_sae_id,
                "slave SAE is not authorized for key"
            );
            return Err(ApiError::unauthorized());
        }

        if key_data.master_sae_id != master_sae_id {
            warn!(
                requested_key_id = %key_request.key_id,
                expected_master_sae_id = key_data.master_sae_id,
                actual_master_sae_id = master_sae_id,
                "master SAE does not match key ownership"
            );
            return Err(ApiError::unauthorized());
        }

        loaded_keys.push((key_request.key_id, key_data));
    }

    let mut response_keys = Vec::with_capacity(loaded_keys.len());

    for (key_id, key_data) in loaded_keys {
        app_state
            .storage
            .delete_key(key_id)
            .await
            .map_err(ApiError::service_unavailable)?;

        response_keys.push(KeyResponse {
            key_id,
            key: STANDARD.encode(key_data.key),
        });
    }

    Ok(Json(KeyContainerResponse {
        keys: response_keys,
        key_container_extension: HashMap::new(),
    }))
}

/// Resolves the calling SAE identity from request metadata and verifies that it exists in the configured topology.
fn resolve_calling_sae_id(
    app_state: &AppState,
    headers: &HeaderMap,
    authenticated_peer: Option<&AuthenticatedPeer>,
) -> Result<String, ApiError> {
    if app_state.config.use_mtls {
        let Some(authenticated_peer) = authenticated_peer else {
            return Err(ApiError::unauthorized());
        };

        debug!(
            sae_id = authenticated_peer.sae_id,
            cert_subject = authenticated_peer.cert_subject,
            "resolved SAE ID from authenticated client certificate"
        );

        if find_kme_for_sae(&app_state.config, &authenticated_peer.sae_id).is_none() {
            warn!(
                sae_id = authenticated_peer.sae_id,
                cert_subject = authenticated_peer.cert_subject,
                "client certificate SAE ID is unknown"
            );
            return Err(ApiError::unauthorized());
        }

        return Ok(authenticated_peer.sae_id.clone());
    }

    let Some(value) = headers.get(SAE_ID_HEADER) else {
        return Err(ApiError::unauthorized());
    };

    let sae_id = value.to_str().map_err(|_| ApiError::unauthorized())?.to_string();

    if find_kme_for_sae(&app_state.config, &sae_id).is_none() {
        warn!(sae_id, "caller SAE ID is unknown");
        return Err(ApiError::unauthorized());
    }

    Ok(sae_id)
}

/// Finds which configured KME owns a given SAE ID.
fn find_kme_for_sae<'a>(config: &'a Config, sae_id: &str) -> Option<&'a str> {
    config.kme_topology.iter().find_map(|(kme_id, peer_config)| {
        peer_config
            .connected_saes
            .iter()
            .any(|candidate| candidate == sae_id)
            .then_some(kme_id.as_str())
    })
}

/// Validates that the requested slave SAE exists in the configured topology.
fn validate_slave_sae_id(config: &Config, slave_sae_id: &str) -> Result<(), ApiError> {
    if find_kme_for_sae(config, slave_sae_id).is_none() {
        return Err(ApiError::bad_request("unknown slave SAE ID"));
    }

    Ok(())
}

/// Validates the subset of ETSI014 key request options currently supported by this implementation.
fn validate_get_key_request(request: &GetKeyRequest) -> Result<(), ApiError> {
    let number = request.number.unwrap_or(1);
    if number == 0 {
        return Err(ApiError::bad_request("number shall be greater than 0"));
    }
    if number > DEFAULT_MAX_KEYS_PER_REQUEST {
        return Err(ApiError::bad_request(format!(
            "number shall be less than or equal to {DEFAULT_MAX_KEYS_PER_REQUEST}"
        )));
    }

    if let Some(size) = request.size {
        if size % 8 != 0 {
            return Err(ApiError::bad_request("size shall be a multiple of 8"));
        }
        if size != DEFAULT_KEY_SIZE_BITS {
            return Err(ApiError::bad_request(format!(
                "size shall be {DEFAULT_KEY_SIZE_BITS}"
            )));
        }
    }

    if !request.additional_slave_sae_ids.is_empty() {
        return Err(ApiError::bad_request(
            "additional_slave_SAE_IDs is not supported",
        ));
    }

    if !request.extension_mandatory.is_empty() {
        return Err(ApiError::bad_request(
            "not all extension_mandatory parameters are supported",
        ));
    }

    if !request.extension_optional.is_empty() {
        return Err(ApiError::bad_request(
            "not all extension_optional request options handled",
        ));
    }

    Ok(())
}

/// Generates a 256-bit key payload directly from the OS cryptographic random source.
fn generate_key_material() -> [u8; 32] {
    let mut key = [0u8; 32];
    getrandom(&mut key).expect("OS randomness should be available");
    key
}
