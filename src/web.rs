use std::net::SocketAddr;
use anyhow::{Result};
use tower::{ServiceBuilder};
use tower_http::{compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer};
use axum::{http, Router, routing::{get}, response::{IntoResponse, Response}, body::{Body, BoxBody}};




// use tokio::signal;
// #[cfg(windows)]
// use signal::windows;
use crate::reader;


pub async fn start() -> Result<()> {
    tracing::debug!("Starting WebServer");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await?;
    Ok(())
}

fn app() -> Router {
    let middleware = ServiceBuilder::new().layer(CompressionLayer::new()).layer(SetSensitiveHeadersLayer::new(std::iter::once(http::header::AUTHORIZATION)));
    Router::new()
        .route("/", get(get_head_handler))
        .layer(middleware.into_inner())

}

// GET routes will also be called for HEAD requests but will have the response body removed.
// You can handle the HEAD method explicitly by extracting `http::Method` from the request.
async fn get_head_handler() -> Response<BoxBody> {

    let x = reader::read().await.unwrap();
    Response::builder()
        .header(http::header::AUTHORIZATION, "PGP")
        .header("X-PRIMUS", "Paul")
        .body(axum::body::boxed(Body::from(x)))
        .unwrap()

}

