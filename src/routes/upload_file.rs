use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use actix_multipart::Multipart;
use actix_web::{post, web, Error, HttpResponse};
use chrono::{DateTime, Local, NaiveDateTime};
use futures_util::StreamExt as _;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::domain::FileRecord;

#[post("v1/files")]
pub async fn upload_file(
    mut payload: Multipart,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();
        let file_name = content_type.get_filename().unwrap();
        let mut path = std::env::current_dir().unwrap();
        path.push(file_name);

        if let Ok(mut file) = std::fs::File::create(&path) {
            while let Some(chunk) = field.next().await {
                file.write_all(&chunk?).unwrap();
            }
            create_file_record(&path, &db).await.unwrap();
        }
    }

    Ok(HttpResponse::Ok().finish())
}

async fn create_file_record(
    path: &PathBuf,
    db: &web::Data<Surreal<Client>>,
) -> surrealdb::Result<()> {
    let filename = path.file_name().unwrap().to_string_lossy().into_owned();
    let size = path.metadata().unwrap().len();
    let mime = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();
    let current_local: DateTime<Local> = Local::now();
    let record = FileRecord {
        filename: filename.clone(),
        size: size,
        mime_type: mime,
        created_at: current_local,
    };
    let _created: Vec<FileRecord> = db.create("files").content(record).await?;

    Ok(())
}
