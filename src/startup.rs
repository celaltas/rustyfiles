use crate::routes::delete_file;
use crate::routes::download_file;
use crate::routes::home_page;
use crate::routes::upload_file;
use actix_web::web::Data;
use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub fn run(listener: TcpListener, client: Surreal<Client>, storage_path:String) -> Result<Server, std::io::Error> {
    let db = Data::new(client);
    let storage_path = Data::new(storage_path);
    let server = HttpServer::new(move || {
        App::new()
            .service(home_page)
            .service(delete_file)
            .service(download_file)
            .service(upload_file)
            .app_data(db.clone())
            .app_data(storage_path.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
