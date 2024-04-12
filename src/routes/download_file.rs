use actix_files::NamedFile;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, web, Error};

#[get("v1/files/{name}")]
pub async fn download_file(name: web::Path<String>, storage_path: web::Data<String>) -> Result<NamedFile, Error> {
    let filename = name.into_inner();
    let mut path = std::env::current_dir().unwrap();
    path.push(storage_path.as_str());
    path.push(filename);
    let file = NamedFile::open(path)?;
    Ok(file.set_content_disposition(ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![],
    }))
}
