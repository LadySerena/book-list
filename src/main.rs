use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

mod log;

fn create_database(name: &str) -> Result<(), postgres::Error> {
    let mut client = Client::connect(&*format!("host=localhost user=postgres password=foobar1234 dbname={}", name), NoTls)?;
    client.batch_execute(r#"CREATE TABLE book (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        authors TEXT[]
    )"#)
}

struct Book {
    title: String,
    authors: Vec<String>,
}


fn populate_database() -> Result<u64, postgres::Error> {
    let book_list: [Book; 2] = [
        Book {
            title: "meow".to_string(),
            authors: vec!["catsworth".to_string()],
        }, Book {
            title: "meow2".to_string(),
            authors: vec!["catsworth2".to_string(), "sir mittens".to_string()],
        }
    ];
    let mut client = Client::connect("host=localhost user=postgres password=foobar1234 dbname=serena_test", NoTls)?;

    client.execute(r#"INSERT INTO book (title, authors) VALUES ($1, $2)"#, &[&book_list[0].title, &book_list[0].authors])?;
    client.execute(r#"INSERT INTO book (title, authors) VALUES ($1, $2)"#, &[&book_list[1].title, &book_list[1].authors])
}


fn main() {
    let attributes = log::LogAttributes::new().unwrap();

    let logger = log::Logger::new(attributes);

    logger.write("hello world");


    create_database("serena_test").unwrap();

    populate_database().unwrap();

    println!("Hello, world!");
}
