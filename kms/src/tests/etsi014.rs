use std::{collections::HashMap, sync::Arc};

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde_json::Value;
use tower::util::ServiceExt;

use crate::{
    AppState,
    config::{Config, KmePeerConfig, StorageConfig, StorageConfigType},
    routes,
    storage::memory::MemoryKeyStorage,
};

fn test_config() -> Config {
    let mut topology = HashMap::new();
    topology.insert(
        "KME_01".to_string(),
        KmePeerConfig {
            connected_saes: vec!["SAE_01".to_string(), "SAE_02".to_string()],
        },
    );
    topology.insert(
        "KME_02".to_string(),
        KmePeerConfig {
            connected_saes: vec!["SAE_03".to_string(), "SAE_04".to_string()],
        },
    );

    Config {
        listen: "127.0.0.1:0".to_string(),
        use_mtls: false,
        storage: StorageConfig {
            typ: StorageConfigType::Memory,
            params: HashMap::new(),
        },
        tls: None,
        kme_topology: topology,
    }
}

fn test_app() -> axum::Router {
    let app_state = Arc::new(AppState {
        storage: Box::new(MemoryKeyStorage::new()),
        config: test_config(),
    });

    routes::build_router(app_state)
}

async fn send_request(
    app: axum::Router,
    request: Request<Body>,
) -> (StatusCode, Value) {
    let response = app
        .oneshot(request)
        .await
        .expect("request should be handled");
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body should be readable");

    let json = if body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&body).expect("body should be valid json")
    };

    (status, json)
}

#[tokio::test]
async fn status_returns_topology_for_master_and_slave() {
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/keys/SAE_03/status")
        .header("x-sae-id", "SAE_01")
        .body(Body::empty())
        .expect("request should build");

    let (status, body) = send_request(test_app(), request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["source_KME_ID"], "KME_01");
    assert_eq!(body["target_KME_ID"], "KME_02");
    assert_eq!(body["master_SAE_ID"], "SAE_01");
    assert_eq!(body["slave_SAE_ID"], "SAE_03");
    assert_eq!(body["key_size"], 256);
}

#[tokio::test]
async fn post_workflow_issues_and_consumes_keys() {
    let app = test_app();

    let issue_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_03/enc_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_01")
        .body(Body::from(r#"{ "number": 2, "size": 256 }"#))
        .expect("request should build");

    let (issue_status, issue_body) = send_request(app.clone(), issue_request).await;

    assert_eq!(issue_status, StatusCode::OK);
    let issued_keys = issue_body["keys"].as_array().expect("keys array");
    assert_eq!(issued_keys.len(), 2);

    let first_key_id = issued_keys[0]["key_ID"]
        .as_str()
        .expect("key ID should be present")
        .to_string();
    let first_key_b64 = issued_keys[0]["key"].as_str().expect("key should be present");
    let decoded = STANDARD.decode(first_key_b64).expect("key should be base64");
    assert_eq!(decoded.len(), 32);

    let retrieve_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_01/dec_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_03")
        .body(Body::from(format!(
            r#"{{ "key_IDs": [{{ "key_ID": "{}" }}] }}"#,
            first_key_id
        )))
        .expect("request should build");

    let (retrieve_status, retrieve_body) = send_request(app.clone(), retrieve_request).await;

    assert_eq!(retrieve_status, StatusCode::OK);
    let retrieved_keys = retrieve_body["keys"].as_array().expect("keys array");
    assert_eq!(retrieved_keys.len(), 1);
    assert_eq!(retrieved_keys[0]["key_ID"], first_key_id);
    assert_eq!(retrieved_keys[0]["key"], first_key_b64);

    let missing_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_01/dec_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_03")
        .body(Body::from(format!(
            r#"{{ "key_IDs": [{{ "key_ID": "{}" }}] }}"#,
            first_key_id
        )))
        .expect("request should build");

    let (missing_status, missing_body) = send_request(app, missing_request).await;

    assert_eq!(missing_status, StatusCode::BAD_REQUEST);
    assert_eq!(
        missing_body["message"],
        "one or more keys specified are not found on KME"
    );
}

#[tokio::test]
async fn get_shortcuts_work_for_single_key_flow() {
    let app = test_app();

    let issue_request = Request::builder()
        .method("GET")
        .uri("/api/v1/keys/SAE_03/enc_keys?number=1&size=256")
        .header("x-sae-id", "SAE_01")
        .body(Body::empty())
        .expect("request should build");

    let (issue_status, issue_body) = send_request(app.clone(), issue_request).await;

    assert_eq!(issue_status, StatusCode::OK);
    let key_id = issue_body["keys"][0]["key_ID"]
        .as_str()
        .expect("key ID should be present");

    let retrieve_request = Request::builder()
        .method("GET")
        .uri(format!("/api/v1/keys/SAE_01/dec_keys?key_ID={key_id}"))
        .header("x-sae-id", "SAE_03")
        .body(Body::empty())
        .expect("request should build");

    let (retrieve_status, retrieve_body) = send_request(app, retrieve_request).await;

    assert_eq!(retrieve_status, StatusCode::OK);
    assert_eq!(retrieve_body["keys"][0]["key_ID"], key_id);
}

#[tokio::test]
async fn wrong_slave_or_master_cannot_retrieve_key() {
    let app = test_app();

    let issue_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_03/enc_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_01")
        .body(Body::from(r#"{ "number": 1, "size": 256 }"#))
        .expect("request should build");

    let (issue_status, issue_body) = send_request(app.clone(), issue_request).await;

    assert_eq!(issue_status, StatusCode::OK);
    let key_id = issue_body["keys"][0]["key_ID"]
        .as_str()
        .expect("key ID should be present");

    let wrong_slave_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_01/dec_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_04")
        .body(Body::from(format!(
            r#"{{ "key_IDs": [{{ "key_ID": "{}" }}] }}"#,
            key_id
        )))
        .expect("request should build");

    let (wrong_slave_status, wrong_slave_body) = send_request(app.clone(), wrong_slave_request).await;

    assert_eq!(wrong_slave_status, StatusCode::UNAUTHORIZED);
    assert_eq!(wrong_slave_body, Value::Null);

    let wrong_master_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_02/dec_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_03")
        .body(Body::from(format!(
            r#"{{ "key_IDs": [{{ "key_ID": "{}" }}] }}"#,
            key_id
        )))
        .expect("request should build");

    let (wrong_master_status, wrong_master_body) =
        send_request(app.clone(), wrong_master_request).await;

    assert_eq!(wrong_master_status, StatusCode::UNAUTHORIZED);
    assert_eq!(wrong_master_body, Value::Null);

    let valid_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_01/dec_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_03")
        .body(Body::from(format!(
            r#"{{ "key_IDs": [{{ "key_ID": "{}" }}] }}"#,
            key_id
        )))
        .expect("request should build");

    let (valid_status, _) = send_request(app, valid_request).await;

    assert_eq!(valid_status, StatusCode::OK);
}

#[tokio::test]
async fn helper_rules_and_validation_failures_are_exposed_over_http() {
    let app = test_app();

    let unknown_slave_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_99/enc_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_01")
        .body(Body::from(r#"{ "number": 1, "size": 256 }"#))
        .expect("request should build");

    let (unknown_slave_status, unknown_slave_body) = send_request(app.clone(), unknown_slave_request).await;

    assert_eq!(unknown_slave_status, StatusCode::BAD_REQUEST);
    assert_eq!(unknown_slave_body["message"], "unknown slave SAE ID");

    let invalid_size_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_03/enc_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_01")
        .body(Body::from(r#"{ "number": 1, "size": 255 }"#))
        .expect("request should build");

    let (invalid_size_status, invalid_size_body) = send_request(app.clone(), invalid_size_request).await;

    assert_eq!(invalid_size_status, StatusCode::BAD_REQUEST);
    assert_eq!(invalid_size_body["message"], "size shall be a multiple of 8");

    let unsupported_extension_request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/SAE_03/enc_keys")
        .header("content-type", "application/json")
        .header("x-sae-id", "SAE_01")
        .body(Body::from(
            r#"{ "extension_mandatory": [ { "vendor_mode": "strict" } ] }"#,
        ))
        .expect("request should build");

    let (unsupported_extension_status, unsupported_extension_body) =
        send_request(app, unsupported_extension_request).await;

    assert_eq!(unsupported_extension_status, StatusCode::BAD_REQUEST);
    assert_eq!(
        unsupported_extension_body["message"],
        "not all extension_mandatory parameters are supported"
    );
}
