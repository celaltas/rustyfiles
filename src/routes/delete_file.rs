use actix_web::{delete, web, HttpResponse, Responder};




#[delete("v1/files/{id}")]
pub async fn delete_file(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("delete file {}", id))
}