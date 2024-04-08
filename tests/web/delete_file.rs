use crate::helpers::spawn_test_server;
use chrono::Local;
use rustyfiles::domain::{DBFileRecord, FileRecord};

#[actix_web::test]
async fn test_delete_file_record_exist() {
    let app = spawn_test_server().await;
    let http_client = reqwest::Client::new();
    let _record:Option<DBFileRecord> = app.db_client
        .create(("files", "testfile123"))
        .content(FileRecord {
            filename: "TestFile123".to_string(),
            size: 123,
            mime_type: "text/plain".to_string(),
            created_at: Local::now(),
        })
        .await
        .expect("failed to create test file record");
    
    let response = http_client
        .delete(&format!("{}/v1/files/{}", &app.address, "testfile123"))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 200);
}

#[actix_web::test]
async fn test_delete_file_record_not_found() {
    let app = spawn_test_server().await;
    let http_client = reqwest::Client::new();
    let record_id = 1;
    let response = http_client
        .delete(&format!("{}/v1/files/{}", &app.address, record_id))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 404);
}
