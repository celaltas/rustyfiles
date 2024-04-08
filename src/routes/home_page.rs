use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tera::{Context, Tera};

use crate::domain::DBFileRecord;

#[get("/")]
pub async fn home_page(
    _req: HttpRequest,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut base_dir = std::env::current_dir().unwrap();
    base_dir.push("static");
    let static_dir = format!("{}/**/*", base_dir.to_str().unwrap());
    let tera = Tera::new(&static_dir).expect("failed to new tera");
    let sql = "SELECT * FROM files ORDER BY created_at DESC LIMIT 100 START 1";
    let mut result = db.query(sql).await.unwrap();
    let records: Vec<DBFileRecord> = result.take(0).unwrap();
    ctx.insert("records", &records);
    let rendered = tera
        .render("home_page.html", &ctx)
        .expect("Failed to render template");
    Ok(HttpResponse::Ok().body(rendered))
}
