use axum::{extract::{Query, Json}, response::{Response, IntoResponse}, routing::get, Router};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::io::Error;
use reqwest::StatusCode;
use std::collections::HashMap;

mod questions;
mod api;

async fn get_questions(
    params: Query<HashMap<String, String>>,
    store: api::Store
) -> Result<Json<Vec<questions::Question>>, Response> {
    let mut start = 0;
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Could not parse start");
    }
    let res: Vec<questions::Question> = store.questions.values().cloned().collect();

    Ok(Json(res))
}

#[tokio::main]
async fn main() {
    let store = api::Store::new();
    //Function for route "/questions"
    let get_items = Router::new().route("/questions", get(get_questions));

    //Base addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //Serve on the addr
    axum::Server::bind(&addr)
        .serve(get_items.into_make_service())
        .await
        .unwrap()
}
