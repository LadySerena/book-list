use crate::log::Logger;
use postgres::{Config, NoTls};
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
}

impl Book {
    pub fn new(title: String, authors: Vec<String>) -> Self {
        Self { title, authors }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Entry {
    book: Book,
    medium: String,
}

impl Entry {
    pub fn new(book: Book, medium: String) -> Self {
        Self { book, medium }
    }
}

#[derive(Clone)]
pub struct State {
    pub db_client: Pool<PostgresConnectionManager<NoTls>>,
    pub logger_ref: Arc<Logger>,
}

#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    username: String,
    password: String,
}

impl State {
    pub fn new(logger: Logger) -> Self {

        //note this was originally /config/database.yaml
        //todo make creds file path configurable
        let file = File::open("./local-config.yaml").unwrap();
        let config: DatabaseConfig = serde_yaml::from_reader(file).unwrap();
        let mut db_info = Config::new();
        db_info.password(config.password);
        db_info.user(config.username.as_str());
        db_info.dbname("booklist-db");
        db_info.host("localhost");//note this was originally database since this was running in docker-compose
        //todo make hostname configurable


        let manager = PostgresConnectionManager::new(db_info, NoTls);

        let db_client = Pool::new(manager).unwrap();

        let logger_ref = Arc::new(logger);

        Self {
            db_client,
            logger_ref,
        }
    }
}
