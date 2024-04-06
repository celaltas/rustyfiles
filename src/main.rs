use std::net::TcpListener;
use rustyfiles::{configuration::{connect_db, get_configuration}, startup::run};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("{}:{}",configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    let client = connect_db(configuration.database).await.expect("Failed to connect surrealdb");
    run(listener, client)?.await
}
