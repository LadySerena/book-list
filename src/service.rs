use std::borrow::Borrow;
use tide::{Body, Request, Response, StatusCode};
use crate::data_models;

pub async fn liveness(_req: Request<data_models::State>) -> tide::Result {
    Ok(Response::new(StatusCode::Ok))
}

pub async fn readiness(_req: Request<data_models::State>) -> tide::Result { Ok(Response::new(StatusCode::Ok)) }

pub async fn post_book(mut req: Request<data_models::State>) -> tide::Result {

    let body:data_models::Book = req.body_json().await?;

    req.state().db_client.get()?.execute(
        "INSERT INTO book (title, authors) VALUES ($1,$2)",
        &[&body.title,&body.authors],
    )?;

    Ok(Response::new(StatusCode::Ok))
}

pub async fn get_book(req: Request<data_models::State>) -> tide::Result {

    req.state().logger_ref.debug("i'm here");

    let title = req.url().query_pairs();

    let mut foo:String = "".to_string();

    for i in title.take(1) {
        foo = i.1.to_string()
    }

    for row in req.state().db_client.get()?.query(
        "SELECT id FROM book WHERE title = $1",
        &[&foo]
    )? {
        let db_title:i32 = row.get("id");
        req.state().logger_ref.debug(format!("title: {}", db_title).as_str())
    }


    Ok(Response::new(StatusCode::Ok))

}