use crate::helpers::spawn_test_server;
use chrono::Local;
use reqwest::multipart;
use rustyfiles::domain::{DBFileRecord, FileRecord};
use std::{
    fs::{self, File},
    io::{Read, Write},
};
use tempfile::NamedTempFile;
use uuid::Uuid;

#[actix_web::test]
async fn test_delete_file_and_its_record() {
    let app = spawn_test_server().await;
    let http_client = reqwest::Client::new();

    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "This is a temp file").unwrap();
    let file_path = temp_file.path();
    let file_name = Uuid::new_v4().to_string();

    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let part = multipart::Part::bytes(contents).file_name(file_name.clone());
    let form = multipart::Form::new().part("file", part);

    let response = http_client
        .post(&format!("{}/v1/files", &app.address))
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    let mut created_file_path = std::env::current_dir().unwrap();
    created_file_path.push(app.test_storage.as_str());
    created_file_path.push(file_name.clone());

    assert_eq!(response.status().as_u16(), 200);

    let sql = "SELECT * FROM files WHERE filename=$filename";
    let mut result = app
        .db_client
        .query(sql)
        .bind(("filename", file_name.clone()))
        .await
        .unwrap();
    let created: Option<DBFileRecord> = result.take(0).unwrap();
    assert!(created.is_some());

    let id = created.unwrap().id.id.to_raw();
    let response = http_client
        .delete(&format!("{}/v1/files/{}", &app.address, id))
        .send()
        .await
        .expect("Failed to execute request.");
    let attr = fs::metadata(created_file_path);

    assert_eq!(response.status().as_u16(), 200);
    assert!(attr.is_err());




}
#[actix_web::test]
async fn test_delete_file_record_exist() {
    let app = spawn_test_server().await;
    let http_client = reqwest::Client::new();
    let _record: Option<DBFileRecord> = app
        .db_client
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
