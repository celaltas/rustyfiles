use actix_web::{delete, web, Error, HttpResponse};

#[delete("v1/files/{id}")]
pub async fn delete_file(path: web::Path<u64>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("delete file {}", id)))
}
