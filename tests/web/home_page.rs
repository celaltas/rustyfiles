use crate::helpers::spawn_test_server;

#[actix_web::test]
async fn test_home_page() {
    let app = spawn_test_server().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&app.address)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
}