use std::sync::Arc;
use opentelemetry::{global, Key};
use opentelemetry::metrics::Counter;
use opentelemetry_prometheus::PrometheusExporter;
use prometheus::{Encoder, TextEncoder};
use tide::{Body, Middleware, Next, Request, Response, StatusCode};
use crate::log::Logger;

const METRICS_ROUTE: &str = "/metrics";
const ROUTE_KEY: Key = Key::from_static_str("http_route");

pub struct TelemetryMiddleware {
    logger: Logger,
    exporter: PrometheusExporter,
    request_count: Counter<u64>,
    encoder: TextEncoder,
}

impl TelemetryMiddleware {
    pub fn new(logger: Logger) -> Self {
        let exporter = opentelemetry_prometheus::exporter().init();
        let meter = global::meter("middleware");

        let request_count = meter
            .u64_counter("http_server_requests_count")
            .with_description("total request count")
            .init();

        let encoder = TextEncoder::new();

        Self {
            logger,
            exporter,
            request_count,
            encoder
        }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for TelemetryMiddleware {
    async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> tide::Result {
        if request.url().path() == METRICS_ROUTE {
            let metric_families = self.exporter.registry().gather();
            let mut result = Vec::new();
            self.encoder.encode(&metric_families, &mut result)?;
            let mut res = Response::new(StatusCode::Ok);
            res.set_content_type(tide::http::mime::PLAIN);
            res.set_body(Body::from_bytes(result));
            Ok(res)
        } else {
            let mut logger = self.logger.clone();
            logger.trace("request received");
            let labels = vec![ROUTE_KEY.string(request.url().path().to_string())];
            self.request_count.add(1, &labels);
            let res = next.run(request).await;
            Ok(res)
        }


    }
}