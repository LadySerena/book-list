use crate::service::liveness;

mod log;
mod data_models;
mod service;
mod middleware;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let attributes = log::LogAttributes::new().unwrap();

    let logger = log::Logger::new(attributes);

    let telemetry_middleware = middleware::TelemetryMiddleware::new(logger);

    let mut app = tide::new();
    app.with(telemetry_middleware);
    app.at("/livez").get(liveness);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
