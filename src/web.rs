use std::net::SocketAddr;
use anyhow::{Result};
use tower::{ServiceBuilder};
use tower_http::{compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer};
use axum::{http, Router, routing::{get}, response::{IntoResponse, Response}, body::{Body, BoxBody}, extract, Json};
use serde::{Serialize, Deserialize};


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
        .route("/pgp/ping", get(ping))
        .route("/pgp/public", get(public))
        .route("/pgp/private", get(private))
        .layer(middleware.into_inner())
}

async fn private() -> Response<BoxBody> {
    let x = reader::read().await.unwrap();
    Response::builder()
        .header(http::header::AUTHORIZATION, "PGP")
        .header("X-PRIMUS", "Paul")
        .header("Content-Type", "text/plain")
        .body(axum::body::boxed(Body::from(x)))
        .unwrap()
}

async fn public() -> Response<BoxBody> {
    let x = reader::read().await.unwrap();
    Response::builder()
        .header(http::header::AUTHORIZATION, "PGP")
        .header("X-PRIMUS", "Paul")
        .header("Content-Type", "text/plain")
        .body(axum::body::boxed(Body::from(x)))
        .unwrap()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Ping<'a> {
    status: &'a str,
}

async fn ping() -> axum::Json<Ping<'static>> {
    // Response::builder().header("ContentType", "application/json")
    //     .body(Ping {status: String::from("Ok")}).unwrap()
    Json::from(Ping {status: "Ok"})
}
