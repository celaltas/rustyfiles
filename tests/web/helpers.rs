use rustyfiles::configuration::{connect_db, get_configuration};
use std::net::TcpListener;
use surrealdb::{engine::remote::ws::Client, Surreal};

pub struct TestApp {
    pub address: String,
    pub db_client: Surreal<Client>,
    pub test_storage:String,
}

pub async fn spawn_test_server() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.dbname = "test_files".to_string();
    let client = connect_db(configuration.database)
        .await
        .expect("Failed to connect surrealdb");
    let server = rustyfiles::startup::run(listener, client.clone(), "tests/expected".to_string())
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_client: client,
        test_storage:"tests/expected".to_string(),
    }
}