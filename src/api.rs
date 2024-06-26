use crate::*;

//Get questions based on browser query
#[debug_handler]
pub async fn get_questions(
    State(AppState { store, .. }): State<AppState>,
) -> Result<impl IntoResponse, FaqError> {
    //let pagination = extract_pagination(params);

    let res: Vec<Question> = match store.get_questions().await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    Ok(Json(res))
}

//Add question to storage
#[debug_handler]
// pub async fn add_question(
//     State(AppState{store, ..}): State<AppState>,
//     Json(question): Json<Question>,
// ) -> Result<impl IntoResponse, FaqError> {
//     store.add_question(question).await;
//     Ok(Json("Question Added"))
// }

pub async fn add_question(
    State(AppState { store, .. }): State<AppState>,
    Json(new_question): Json<NewQuestion>,
) -> Result<impl IntoResponse, FaqError> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(FaqError::DatabaseError(e));
    }

    Ok(Json("Question added"))
}

pub async fn update_question(
    State(AppState { store, .. }): State<AppState>,
    Json((question, id)): Json<(Question, i32)>,
) -> Result<impl IntoResponse, FaqError> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return Err(FaqError::DatabaseError(e)),
    };

    Ok(Json(res))
}

pub async fn delete_question(
    State(AppState { store, .. }): State<AppState>,
    Json(id): Json<i32>,
) -> Result<impl IntoResponse, FaqError> {
    if let Err(e) = store.delete_question(id).await {
        return Err(FaqError::DatabaseError(e));
    };

    Ok(Json(format!("Question {} deleted", id)))
}

pub async fn add_answer(
    State(AppState { store, .. }): State<AppState>,
    Json(new_answer): Json<NewAnswer>,
) -> Result<impl IntoResponse, FaqError> {
    if let Err(e) = store.add_answer(new_answer).await {
        return Err(FaqError::DatabaseError(e));
    }

    Ok(Json("Answer added"))
}
