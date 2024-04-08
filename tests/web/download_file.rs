use crate::helpers::spawn_test_server;

#[actix_web::test]
async fn test_download_file_url_exist() {
    let app = spawn_test_server().await;
    let http_client = reqwest::Client::new();
    let record_id = 1;
    let response = http_client
        .get(&format!("{}/v1/files/{}", &app.address, record_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
}
