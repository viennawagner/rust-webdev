use axum::{routing::get, Router};
use std::net::SocketAddr;

mod questions;
#[tokio::main]
async fn main() {
    //Function for route "/"
    let hello = Router::new().route("/", get(say_hello));

    //Base addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //Serve on the addr
    axum::Server::bind(&addr)
        .serve(hello.into_make_service())
        .await
        .unwrap()
}

async fn say_hello() -> &'static str {
    "Hello, World!"
}
