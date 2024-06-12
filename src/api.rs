use crate::*;

//Get questions based on browser query
#[debug_handler]
pub async fn get_questions(
    State(AppState{store, params}): State<AppState>,
) -> Result<impl IntoResponse, FaqError> {
    //let pagination = extract_pagination(params);
 
    let res: Vec<Question> = match store
        .get_questions()
        .await {
            Ok(res) => res,
            Err(e) => { 
                return Err(e)
            },
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
    State(AppState{store, ..}): State<AppState>,
    Json(new_question): Json<NewQuestion>,
) -> Result<impl IntoResponse, FaqError> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(FaqError::DatabaseError(e));
    }
 
    Ok(Json("Question added"))
}