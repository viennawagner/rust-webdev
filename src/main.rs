use axum::{
    extract::Json,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use std::str::FromStr;

mod api;
mod faqerror;
mod questions;

use api::*;
use questions::*;

//Get questions based on browser query
async fn get_questions(
    store: Store,
) -> Result<Json<Vec<Question>>, Response> {
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
