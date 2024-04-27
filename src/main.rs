use axum::{extract::{Query, Json}, response::{Response, IntoResponse}, routing::get, Router};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::io::Error;
use reqwest::StatusCode;
use std::collections::HashMap;

mod questions;
mod api;
mod faqerror;

use questions::*;
use api::*;
use faqerror::*;

//Get questions based on browser query
async fn get_questions(
    params: Query<HashMap<String, String>>,
    store: Store
) -> Result<Json<Vec<Question>>, Response> {
    //parse query and set the start id accordingly
    let mut start = 0;
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Could not parse start");
    }
    let res: Vec<Question> = store.questions.values().cloned().collect();

    Ok(Json(res))
}

#[tokio::main]
async fn main() {
    let store = Store::new();
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
