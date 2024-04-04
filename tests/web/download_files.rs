use crate::helpers::spawn_test_server;

#[actix_web::test]
async fn test_download_file_url_exist() {
    let address = spawn_test_server();
    let client = reqwest::Client::new();
    let record_id = 1;
    let response = client
        .get(&format!("{}/v1/files/{}", &address, record_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 200);
}
