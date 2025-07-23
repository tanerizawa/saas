use axum::{
    extract::MatchedPath,
    http::Request,
    middleware::{self, Next},
    response::Response,
};
use std::time::Instant;

use crate::infrastructure::monitoring::metrics::{track_http_request, track_http_request_duration};

pub async fn metrics_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };

    let method = req.method().as_str();

    // Use metrics tracking function with closure
    track_http_request_duration(method, &path, || {
        let start = Instant::now();
        let response = next.run(req);
        let duration = start.elapsed();

        // Record duration and other metrics
        let status = response.status().as_u16().to_string();
        track_http_request(method, &path, &status);

        response
    })
}
