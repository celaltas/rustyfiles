use crate::helpers::spawn_test_server;

#[actix_web::test]
async fn test_home_page() {
    let address = spawn_test_server();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
}