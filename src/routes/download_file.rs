use actix_web::{get, web, HttpResponse, Responder};




#[get("v1/files/{id}")]
pub async fn download_file(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("download file {}", id))
}