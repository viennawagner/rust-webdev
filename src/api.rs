use crate::*;

//Get questions based on browser query
#[debug_handler]
pub async fn get_questions(
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
pub async fn add_question(
    State(AppState{store, ..}): State<AppState>,
    Json(question): Json<Question>,
) -> Result<impl IntoResponse, FaqError> {
    store.add_question(question).await;
    Ok(Json("Question Added"))
} 