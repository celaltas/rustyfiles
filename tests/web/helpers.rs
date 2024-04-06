use std::net::TcpListener;

use rustyfiles::configuration::{connect_db, get_configuration};

pub async fn spawn_test_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let client = connect_db(configuration.database).await.expect("Failed to connect surrealdb");
    let server = rustyfiles::startup::run(listener, client).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}