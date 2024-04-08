use crate::domain::DBFileRecord;
use actix_web::{delete, error::ErrorNotFound, web, Error, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[delete("v1/files/{id}")]
pub async fn delete_file(
    path: web::Path<String>,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    match check_and_delete_file_record(id.clone(), db).await {
        Ok(true) => Ok(HttpResponse::Ok().body(format!("delete file {}", id.clone()))),
        Ok(false) => Err(ErrorNotFound("file not found")),
        Err(e) => Err(ErrorNotFound(e.to_string())),
    }
}

async fn check_and_delete_file_record(
    id: String,
    db: web::Data<Surreal<Client>>,
) -> surrealdb::Result<bool> {
    println!("db={:?}", db);
    let file: Option<DBFileRecord> = db.delete(("files", id)).await?;
    println!("{:?}", file);
    Ok(file.is_some())
}
