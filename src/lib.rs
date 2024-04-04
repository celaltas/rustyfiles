use std::net::TcpListener;

use chrono::{format, NaiveDateTime};
use tera::{Context, Tera};

use actix_web::{
    delete, dev::Server, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result
};

#[get("/")]
async fn home_page(_req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let tera = Tera::new("static/**/*").expect("failed to new tera");
    let mut records = vec![];
    records.push(FileRecord {
        id: 1,
        filename: "test.txt".to_string(),
        size: 1024,
        mime_type: "text/plain".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    });
    records.push(FileRecord {
        id: 2,
        filename: "test.png".to_string(),
        size: 1024,
        mime_type: "image/png".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    });
    records.push(FileRecord {
        id: 3,
        filename: "test.pdf".to_string(),
        size: 1024,
        mime_type: "application/pdf".to_string(),
        created_at: chrono::Utc::now().naive_utc(),
    });

    ctx.insert("records", &records);

    let rendered = tera
        .render("home_page.html", &ctx)
        .expect("Failed to render template");
    HttpResponse::Ok().body(rendered)
}

#[delete("v1/files/{id}")]
async fn delete_file(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("delete file {}", id))
}

#[get("v1/files/{id}")]
async fn download_file(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("download file {}", id))
}

#[derive(serde::Serialize)]
struct FileRecord {
    id: u64,
    filename: String,
    size: u64,
    mime_type: String,

    #[serde(with = "chorono_serde")]
    created_at: NaiveDateTime,
}

mod chorono_serde {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(home_page)
            .service(delete_file)
            .service(download_file)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
