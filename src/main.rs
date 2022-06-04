use crate::service::liveness;

mod log;
mod data_models;
mod service;
mod middleware;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let attributes = log::LogAttributes::new().unwrap();

    let mut logger = log::Logger::new(attributes);

    logger.trace("hello world");

    logger.debug("hello world");

    logger.info("hello world");

    logger.warn("hello world");

    logger.error("hello world");

    logger.fatal("hello world");

    let mut app = tide::new();
    app.at("/livez").get(liveness);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
