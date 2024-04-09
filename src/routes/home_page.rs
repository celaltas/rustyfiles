use actix_web::{
    get,
    web::{self, Query},
    Error, HttpResponse,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tera::{Context, Tera};

use crate::domain::DBFileRecord;

#[derive(Debug, serde::Deserialize)]
struct RecordCount {
    number_of_records: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Pagination {
    pagesize: Option<i64>,
    page: Option<i64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: Some(1),
            pagesize: Some(10),
        }
    }
}

#[get("/")]
pub async fn home_page(
    mut info: web::Query<Pagination>,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, Error> {
    let _ = validate_or_default(&mut info)?;
    let mut ctx = Context::new();
    let mut base_dir = std::env::current_dir().unwrap();
    base_dir.push("static");
    let static_dir = format!("{}/**/*", base_dir.to_str().unwrap());
    let tera = Tera::new(&static_dir).expect("failed to new tera");
    let page = info.page.unwrap();
    let limit = info.pagesize.unwrap();
    let start = (page - 1) * limit;

    let sql = "
    SELECT count() AS number_of_records FROM files GROUP ALL;
    SELECT * FROM files ORDER BY created_at DESC LIMIT $limit START $start;";
    let mut result = db
        .query(sql)
        .bind(("limit", limit))
        .bind(("start", start))
        .await
        .unwrap();
    let count: Option<RecordCount> = result.take(0).unwrap();
    let total_pages = count.unwrap().number_of_records / limit;
    let records: Vec<DBFileRecord> = result.take(1).unwrap();
    ctx.insert("records", &records);
    ctx.insert("pagesize", &info.pagesize);
    ctx.insert("page", &info.page);
    ctx.insert("total_pages", &total_pages);

    let rendered = tera
        .render("home_page.html", &ctx)
        .expect("Failed to render template");
    Ok(HttpResponse::Ok().body(rendered))
}

fn validate_or_default(info: &mut web::Query<Pagination>) -> Result<(), Error> {
    if let Some(pagesize) = info.pagesize {
        if pagesize <= 0 {
            return Err(actix_web::error::ErrorBadRequest(
                "pagesize must be positive",
            ));
        }
    } else {
        info.pagesize = Some(10);
    }
    if let Some(page) = info.page {
        if page <= 0 {
            return Err(actix_web::error::ErrorBadRequest("page must be positive"));
        }
    } else {
        info.page = Some(1);
    }
    if info.pagesize.is_none() && info.page.is_none() {
        *info = Query(Pagination::default());
    }
    Ok(())
}
