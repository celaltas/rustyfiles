use actix_web::{get, HttpRequest, HttpResponse, Responder};
use tera::{Context, Tera};

use crate::domain::FileRecord;




#[get("/")]
pub async fn home_page(_req: HttpRequest) -> impl Responder {
    let mut ctx = Context::new();
    let tera = Tera::new("../static/**/*").expect("failed to new tera");
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