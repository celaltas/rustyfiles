use actix_web::{get, web, HttpRequest, HttpResponse, Error};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tera::{Context, Tera};

use crate::domain::FileRecord;

#[get("/")]
pub async fn home_page(_req: HttpRequest, db: web::Data<Surreal<Client>>) -> Result<HttpResponse,Error> {
    let mut ctx = Context::new();
    let tera = Tera::new("../static/**/*").expect("failed to new tera");
    let sql = "SELECT * FROM files ORDER BY created_at DESC LIMIT 10 START 1";
    let mut result = db.query(sql).await.unwrap();
    let records:Vec<FileRecord> = result.take(0).unwrap();
    ctx.insert("records", &records);
    let rendered = tera
        .render("home_page.html", &ctx)
        .expect("Failed to render template");
    Ok(HttpResponse::Ok().body(rendered))
}
