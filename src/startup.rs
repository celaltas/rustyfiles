use std::net::TcpListener;
use crate::routes::home_page;
use crate::routes::delete_file;
use crate::routes::download_file;
use actix_web::{dev::Server, App, HttpServer};



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