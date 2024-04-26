use axum::{extract::Json, response::{Response, IntoResponse}, routing::get, Router};
use std::net::SocketAddr;
use std::str::FromStr;

mod questions;

async fn get_questions() -> Result<Json<questions::Question>, Response> {
    let question = questions::Question::new(
        questions::QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string())),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => {
            Err(questions::InvalidId.into_response())
        },
        Ok(_) => {
            Ok(Json(question))
        }
    }
}

#[tokio::main]
async fn main() {
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
