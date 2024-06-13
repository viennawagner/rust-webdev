use axum::body::Body;
use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use std::collections::HashMap;
use std::io::Error;
use std::str::FromStr;

mod answer;
mod api;
mod faqerror;
mod questions;
mod store;
mod utils;

use answer::*;
use api::*;
use faqerror::*;
use questions::*;
use store::*;
use utils::*;

#[derive(Clone)]
struct AppState {
    store: Store,
}

impl AppState {
    fn new(store: Store) -> Self {
        Self { store }
    }
}

//Set pagination info from the given parameters
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, FaqError> {
    // Could be improved in the future
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            // Takes the "limit" parameter in the query
            // and tries to convert it to a number
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<u32>()
                    .map_err(FaqError::ParseError)?,
            ),
            // Takes the "offset" parameter in the query
            // and tries to convert it to a number
            offset: params
                .get("offset")
                .unwrap()
                .parse::<u32>()
                .map_err(FaqError::ParseError)?,
        });
    }

    Err(FaqError::MissingParameters)
}

#[tokio::main]
async fn main() {
    let store = store::Store::new("postgres://default:1234@localhost:5432/postgres").await;

    let state = AppState::new(store);
    //Function for get methods on "/questions"
    let get_route = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions", post(add_question))
        .route("/questions", put(update_question))
        .route("/questions", delete(delete_question))
        .route("/answers", post(add_answer))
        .with_state(state);

    //Base addr
    let addr = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    //Serve on the addr
    axum::serve(addr, get_route).await.unwrap()
}
