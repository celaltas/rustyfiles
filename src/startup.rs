use crate::routes::delete_file;
use crate::routes::download_file;
use crate::routes::home_page;
use crate::routes::upload_file;
use actix_web::web::Data;
use actix_web::{dev::Server, App, HttpServer};
use std::net::TcpListener;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub fn run(listener: TcpListener, client: Surreal<Client>) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(home_page)
            .service(delete_file)
            .service(download_file)
            .service(upload_file)
            .app_data(Data::new(client.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
