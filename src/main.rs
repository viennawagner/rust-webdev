use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;

mod api;
mod faqerror;
mod questions;
mod utils;

use api::*;
use questions::*;
use faqerror::*;
use utils::*;
 
#[derive(Clone)]
struct AppState {
    store: Store,
    params: HashMap<String, String>,
}

impl AppState {
    fn new(store: Store, params: HashMap<String, String>) -> Self {
        Self {
            store,
            params,
        }
    }
}
 
//Set pagination info from the given parameters
fn extract_pagination(
    params: HashMap<String, String>
) -> Result<Pagination, FaqError> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(FaqError::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(FaqError::ParseError)?,
        });
    }
 
    Err(FaqError::MissingParameters)
}

//Get questions based on browser query
#[debug_handler]
async fn get_questions(
    State(AppState{store, params}): State<AppState>,
) -> Result<impl IntoResponse, FaqError> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect();
        let res = &res[pagination.start..pagination.end];
        Ok(Json(res.to_owned()))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[0..res.len()];
        Ok(Json(res.to_owned()))
    }
} 

//Get questions add question to storage
#[debug_handler]
async fn add_question(
    State(AppState{store, ..}): State<AppState>,
    Json(question): Json<Question>,
) -> Result<impl IntoResponse, FaqError> {
    store.add_question(question).await;
    Ok(Json("Question Added"))
} 

#[tokio::main]
async fn main() {
    let store = Store::new();
    let params = HashMap::from([
        ("start".to_string(), "0".to_string()),
        ("end".to_string(), "10".to_string()),
    ]);

    let state = AppState::new(store, params);
    //Function for get method on "/questions"
    let get_route = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions/add", post(add_question))
        .with_state(state);

    //Base addr
    let addr = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    //Serve on the addr
    axum::serve(addr, get_route)
        .await
        .unwrap()
}
