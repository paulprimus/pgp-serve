use anyhow::Result;
use axum::{
    body::{Body, BoxBody},
    extract, http,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer};

use crate::gpg;

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
    let middleware =
        ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                http::header::AUTHORIZATION,
            )));
    Router::new()
        .route("/pgp/ping", get(ping))
        .route("/pgp/public", get(public))
        .route("/pgp/private", get(private))
        .layer(middleware.into_inner())
}

async fn private() -> Response<BoxBody> {
    let x = gpg::read(gpg::KeyTyp::Private).await.unwrap();
    Response::builder()
        .header(http::header::AUTHORIZATION, "PGP")
        .header("X-PRIMUS", "Paul")
        .header("Content-Type", "text/plain")
        .body(axum::body::boxed(Body::from(x)))
        .unwrap()
}

async fn public() -> Response<BoxBody> {
    let x = gpg::read(gpg::KeyTyp::Public).await.unwrap();
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
    Json::from(Ping { status: "Ok" })
}
