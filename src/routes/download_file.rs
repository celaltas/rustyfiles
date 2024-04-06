use actix_web::{get, web, Error, HttpResponse};




#[get("v1/files/{id}")]
pub async fn download_file(path: web::Path<u64>) -> Result<HttpResponse,Error> {
    let id = path.into_inner();
    Ok(HttpResponse::Ok().body(format!("download file {}", id)))
}