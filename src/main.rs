use std::net::SocketAddr;

use axum::extract::Query;
use axum::routing::get;
use axum::{Router, Server};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    name: String,
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(greeting));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to initialize the Local Library Website Server");
}

async fn greeting(Query(params): Query<Params>) -> String {
    format!("Hello, {}!", params.name)
}
