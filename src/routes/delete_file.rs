use std::fs;

use crate::domain::DBFileRecord;
use actix_web::{delete, error::ErrorNotFound, web, Error, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[delete("v1/files/{id}")]
pub async fn delete_file(
    path: web::Path<String>,
    db: web::Data<Surreal<Client>>,
    storage_path: web::Data<String>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let file: Option<DBFileRecord> = db
        .delete(("files", id.clone()))
        .await
        .map_err(|err| ErrorNotFound(err.to_string()))?;
    match file {
        Some(record) => {
            let filename = record.filename;
            let mut path = std::env::current_dir().unwrap();
            path.push(storage_path.as_str());
            path.push(filename);

            match fs::remove_file(path) {
                Ok(_) => Ok(HttpResponse::Ok().body(format!("delete file {}", id.clone()))),
                Err(err) => Err(ErrorNotFound(err.to_string())),
            }
        }
        None => Err(ErrorNotFound("file not found")),
    }
}
