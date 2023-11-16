#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, routing::get, response::{Html, IntoResponse}, extract::Query};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello: Router = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("handler_hello");
    let name: &str = params.name.as_deref().unwrap_or("World");

    println!("name: {}", name);

    Html(format!("<h1>Hello, {name}!</h1>"));
}

