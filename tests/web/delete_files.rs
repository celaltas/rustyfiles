use crate::helpers::spawn_test_server;



#[actix_web::test]
async fn test_delete_file() {
    let address = spawn_test_server();
    let client = reqwest::Client::new();
    let record_id = 1;
    let response = client
        .delete(&format!("{}/v1/files/{}", &address, record_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
