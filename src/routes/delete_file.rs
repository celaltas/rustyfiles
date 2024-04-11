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
            let path = std::env::current_dir().unwrap();
            let _ = path.join(storage_path.as_str()).join(filename);

            fs::remove_file(path)
                .map(|_| HttpResponse::Ok().body(format!("delete file {}", id)))
                .map_err(|err| ErrorNotFound(err.to_string()))
        }
        None => Err(ErrorNotFound("file not found")),
    }
}
