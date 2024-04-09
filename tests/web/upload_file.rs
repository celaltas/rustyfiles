use crate::helpers::spawn_test_server;
use reqwest::multipart;
use std::{
    fs::{self, File},
    io::{Read, Write},
};
use tempfile::NamedTempFile;

#[actix_web::test]
async fn test_upload_file_with_multipart() {
    let app = spawn_test_server().await;
    let client = reqwest::Client::new();
    let mut temp_file = NamedTempFile::new().unwrap();
    write!(temp_file, "This is a temp file").unwrap();
    let file_path = temp_file.path();
    let file_name = "test.txt";

    let mut file = File::open(file_path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let content_length: u64 = contents.len().try_into().unwrap();

    let part = multipart::Part::bytes(contents).file_name(file_name);
    let form = multipart::Form::new().part("file", part);

    let response = client
        .post(&format!("{}/v1/files", &app.address))
        .multipart(form)
        .send()
        .await
        .expect("Failed to execute request.");

    let mut created_file_path = std::env::current_dir().unwrap();
    created_file_path.push(app.test_storage.as_str());
    created_file_path.push(file_name);
    let attr = fs::metadata(created_file_path);
    assert!(attr.is_ok());
    let attr = attr.unwrap();
    assert!(attr.is_file());
    assert_eq!(attr.len(), content_length);
    assert_eq!(response.status(), 200);

}
