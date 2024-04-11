use crate::domain::DBFileRecord;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    get,
    web::{self, Query},
    Error, HttpResponse,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};
use tera::{Context, Tera};

#[derive(Debug, Deserialize, Serialize)]
struct PageData {
    records: Vec<DBFileRecord>,
    pagination: Pagination,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecordCount {
    number_of_records: i64,
}

impl Default for RecordCount {
    fn default() -> Self {
        RecordCount {
            number_of_records: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Pagination {
    pagesize: Option<i64>,
    page: Option<i64>,
    total_pages: Option<i64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: Some(1),
            pagesize: Some(10),
            total_pages: Some(0),
        }
    }
}

#[get("/")]
pub async fn home_page(
    mut info: web::Query<Pagination>,
    db: web::Data<Surreal<Client>>,
) -> Result<HttpResponse, Error> {
    let _ = validate_or_default(&mut info)?;
    let tera = setup_tera()?;
    let page_data = get_records(info, &db).await?;
    let mut ctx = Context::new();
    ctx.insert("data", &page_data);
    let rendered = tera
        .render("home_page.html", &ctx)
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;
    Ok(HttpResponse::Ok().body(rendered))
}

fn validate_or_default(info: &mut web::Query<Pagination>) -> Result<(), Error> {
    if let Some(pagesize) = info.pagesize {
        if pagesize <= 0 {
            return Err(ErrorBadRequest("pagesize must be positive"));
        }
    } else {
        info.pagesize = Some(10);
    }
    if let Some(page) = info.page {
        if page <= 0 {
            return Err(ErrorBadRequest("page must be positive"));
        }
    } else {
        info.page = Some(1);
    }
    if info.pagesize.is_none() && info.page.is_none() {
        *info = Query(Pagination::default());
    }
    Ok(())
}

async fn get_records(
    mut info: web::Query<Pagination>,
    db: &web::Data<Surreal<Client>>,
) -> Result<PageData, Error> {
    let page = info
        .page
        .ok_or_else(|| ErrorBadRequest("Missing page parameter"))?;
    let limit = info
        .pagesize
        .ok_or_else(|| ErrorBadRequest("Missing pagesize parameter"))?;
    let start = (page - 1) * limit;

    let sql = "
    SELECT count(*) AS number_of_records FROM files;
    SELECT * FROM files ORDER BY created_at DESC LIMIT $1 OFFSET $2;";

    let mut result = db
        .query(sql)
        .bind(&limit)
        .bind(&start)
        .await
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    let count: Option<RecordCount> = result
        .take(0)
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    if let Some(count) = count {
        info.total_pages = Some(count.number_of_records / limit)
    }

    let records: Vec<DBFileRecord> = result
        .take(1)
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    Ok(PageData {
        records,
        pagination: info.into_inner(),
    })
}

fn setup_tera() -> Result<Tera, Error> {
    let current_dir = std::env::current_dir().map_err(|err| {
        ErrorInternalServerError(format!("Failed to get current directory: {}", err))
    })?;

    let mut base_dir = current_dir;
    base_dir.push("static");

    let static_dir = format!("{}/**/*", base_dir.to_str().unwrap());

    Tera::new(&static_dir)
        .map_err(|err| ErrorInternalServerError(format!("Failed to initialize Tera: {}", err)))
}
