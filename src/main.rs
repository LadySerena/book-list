use crate::service::{get_book, liveness, post_book, readiness};

mod log;
mod data_models;
mod service;
mod middleware;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let attributes = log::LogAttributes::new().expect("could not create logger");

    let logger = log::Logger::new(attributes);

    let telemetry_middleware = middleware::TelemetryMiddleware::new(logger.clone());

    let mut app = tide::with_state(data_models::State::new(logger.clone()));
    app.with(telemetry_middleware);
    app.at("/livez").get(liveness);
    app.at("/readyz").get(readiness);
    app.at("/book").post(post_book);
    app.at("/book").get(get_book);

    logger.info("initialization finished");

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
