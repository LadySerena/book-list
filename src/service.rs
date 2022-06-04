use tide::{Request, Response, StatusCode};

pub async fn liveness(_req: Request<()>) -> tide::Result {
    Ok(Response::new(StatusCode::Ok))
}