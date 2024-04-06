use crate::helpers::spawn_test_server;

#[actix_web::test]
async fn test_upload_file_url_exist() {
    let address = spawn_test_server().await;
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/v1/files", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
}
