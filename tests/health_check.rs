use std::net::TcpListener;
use std::sync::Arc;
use modulite::{HealthCheckResponse, MessageResponse};
use modulite::repository::Repository;

#[tokio::test]
async fn health_check_works() {
    let (address, _) = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let health_response: HealthCheckResponse = response
        .json()
        .await
        .expect("Failed to parse json");
    assert_eq!(health_response.status, "Healthy");
    assert!(health_response.database);
}

#[tokio::test]
async fn get_message_works() {
    let (address, _) = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let message_response: MessageResponse = response
        .json()
        .await
        .expect("Failed to parse json");
    assert_eq!(message_response.message, "Welcome to Rust Server of Modu.lite App!");
}

#[tokio::test]
async fn test_db_connection_works() {
    let (address, repository) = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/test_db", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let response_body = response.text().await.expect("Failed to get response body");
    assert_eq!(response_body, "Database is healthy!");

    let result = repository.check_database_connection().await;
    assert!(result);
}

async fn spawn_app() -> (String, Arc<Repository>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let repository = Repository::new("postgresql://modulite_db_user:vNfd7OfxHBdV77fM9yRdp8XkXrdtXDlU@dpg-cr3ltpt6l47c73a7m3ag-a.oregon-postgres.render.com/modulite_db").await.expect("Failed to connect to DB");
    let server = modulite::run(listener, Arc::new(repository.clone())).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    (format!("http://127.0.0.1:{}", port), Arc::new(repository))
}


//TODO: Make tests for new routes and CRUD operations, fix this tests
