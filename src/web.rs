use std::net::SocketAddr;
use anyhow::{Result};
use tower::{ServiceBuilder};
use tower_http::{compression::CompressionLayer, sensitive_headers::SetSensitiveHeadersLayer};
use axum::{http, Router, routing::{get}, response::{IntoResponse, Response}};

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
async fn get_head_handler() -> Response {
    // it usually only makes sense to special-case HEAD
    // if computing the body has some relevant cost
    // if method == http::Method::HEAD {
    //         ([(http::header::AUTHORIZATION.as_str(), "paul"),("X-PGP", "test") ]).into_response();
    // }

    // then do some computing task in GET
    // do_some_computing_task();
    let x = reader::read().await.unwrap();
    tracing::debug!("{}",x);
    ([(http::header::AUTHORIZATION.as_str(), "paul"),("X-PGP", x.as_str()) ]).into_response()
}

