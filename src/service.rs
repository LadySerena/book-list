use std::borrow::Borrow;
use std::collections::HashMap;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::r2d2::{Error, PooledConnection};

use serde_json::json;
use tide::{Request, Response, StatusCode};
use tide::http::mime;

use crate::data_models::{Book, State};

pub async fn liveness(_req: Request<State>) -> tide::Result {
    Ok(Response::new(StatusCode::Ok))
}

pub async fn readiness(_req: Request<State>) -> tide::Result { Ok(Response::new(StatusCode::Ok)) }

pub async fn post_book(mut req: Request<State>) -> tide::Result {
    let body: Book = req.body_json().await?;

    req.state().db_client.get()?.execute(
        "INSERT INTO book (title, authors) VALUES ($1,$2)",
        &[&body.title, &body.authors],
    )?;

    Ok(Response::new(StatusCode::Ok))
}

pub async fn get_book(req: Request<State>) -> tide::Result {
    req.state().logger_ref.debug("i'm here");

    let query_params: HashMap<_, _> = req.url().query_pairs().into_owned().collect();

    let title = query_params.get("title");

    if title.is_none() {
        Ok(Response::new(StatusCode::BadRequest))
    } else {

        let client = req.state().db_client.get();

        let mut client = match client {
            Ok(client) => {
                client
            },
            Err(error) => {
                return Err(error.into())
            },
        };

        let statement = client.prepare("SELECT id, title, authors FROM book WHERE title = $1 LIMIT=1");

        let statement = match statement {
            Ok(statement) => statement,
            Err(error) => {
                return Err(error.into())
            }
        };

        let rows = req.state().db_client.get()?.query(&statement, &[title.borrow()])?;

        if rows.is_empty() {
            Ok(Response::new(StatusCode::NotFound))
        } else {
            let book = Book {
                title: rows[0].get(1),
                authors: rows[0].get(2),
            };

            let serialized = serde_json::to_string(&book)?;

            let response = tide::Response::builder(StatusCode::Ok)
                .body(serialized)
                .content_type(mime::JSON)
                .build();

            Ok(response)
        }

    }
}