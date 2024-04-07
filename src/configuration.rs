use config::{Config, ConfigError, File, FileFormat};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub dbname: String,
    pub namespace: String,
}

pub fn get_configuration() -> Result<Settings, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let conf_path = base_path.join("configuration/base");
    let builder = Config::builder().add_source(File::new(
        conf_path.to_str().unwrap(),
        FileFormat::Yaml,
    ));
    let config = builder.build()?;
    config.try_deserialize()
}

pub async fn connect_db(settings: DatabaseSettings) -> surrealdb::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(format!("{}:{}", settings.host, settings.port).as_str()).await?;
    db.signin(Root {
        username: settings.username.as_str(),
        password: settings.password.as_str(),
    })
    .await?;
    db.use_ns(settings.namespace)
        .use_db(settings.dbname)
        .await?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use crate::configuration::{connect_db, ApplicationSettings, DatabaseSettings, Settings};

    use super::get_configuration;

    #[test]
    fn test_get_configuration() {
        let res = get_configuration();
        assert!(res.is_ok());
        let expected = Settings {
            application: ApplicationSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            database: DatabaseSettings {
                host: "127.0.0.1".to_string(),
                port: 8000,
                username: "root".to_string(),
                password: "root".to_string(),
                dbname: "test".to_string(),
                namespace: "test".to_string(),
            },
        };

        assert_eq!(res.unwrap(), expected);
    }


    #[tokio::test]
    async fn test_connect_db() {
        let settings = DatabaseSettings {
            host: "127.0.0.1".to_string(),
            port: 8000,
            username: "root".to_string(),
            password: "root".to_string(),
            dbname: "test".to_string(),
            namespace: "test".to_string(),
        };

        let db = connect_db(settings).await;
        println!("{:?}", db);
        assert!(db.is_ok());
        


    }
}
