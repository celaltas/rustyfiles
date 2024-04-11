use actix_multipart::Multipart;
use actix_web::{
    error::ErrorInternalServerError,
    post,
    web::{self, Redirect},
    Error,
};
use chrono::{DateTime, Local};
use futures_util::StreamExt as _;
use std::{fs, io::Write, path::PathBuf};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::domain::{DBFileRecord, FileRecord};

#[post("v1/files")]
pub async fn upload_file(
    mut payload: Multipart,
    db: web::Data<Surreal<Client>>,
    storage_path: web::Data<String>,
) -> Result<Redirect, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();
        let file_name = content_type.get_filename().ok_or(ErrorInternalServerError(
            "Filename can not be read from requets",
        ))?;
        let mut path = std::env::current_dir()?;
        path.push(storage_path.as_str());
        path.push(file_name);

        if let Ok(mut file) = fs::File::create(&path) {
            while let Some(chunk) = field.next().await {
                file.write_all(&chunk?)?;
            }
            create_file_record(&path, &db).await?;
        }
    }

    Ok(Redirect::to("/").see_other())
}

async fn create_file_record(path: &PathBuf, db: &web::Data<Surreal<Client>>) -> Result<(), Error> {
    let filename = path
        .file_name()
        .ok_or(ErrorInternalServerError(
            "Filename can not be get from path",
        ))?
        .to_string_lossy()
        .into_owned();
    let size = path
        .metadata()
        .map_err(|err| ErrorInternalServerError(err.to_string()))?
        .len();
    let mime = mime_guess::from_path(&path)
        .first_or_octet_stream()
        .to_string();
    let current_local = Local::now();
    let record = FileRecord {
        filename: filename.clone(),
        path: path
            .to_str()
            .ok_or(ErrorInternalServerError("File path not found"))?
            .to_string(),
        size: size,
        mime_type: mime,
        created_at: current_local,
    };
    let _created: Vec<DBFileRecord> = db
        .create("files")
        .content(record)
        .await
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    Ok(())
}
