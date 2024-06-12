use crate::*;

#[derive(Debug)]
pub enum FaqError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseError(sqlx::Error),
}

impl std::fmt::Display for FaqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FaqError::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            FaqError::MissingParameters => write!(f, "Missing parameter"),
            FaqError::DatabaseError(e) => write!(f, "Database Error: {}", e),
        }
    }
}
impl IntoResponse for FaqError {
    fn into_response(self) -> Response {
        let body = match self {
            FaqError::ParseError(ref err) => {
                format!("Cannot parse parameter: {}", err)
            }
            FaqError::MissingParameters => "Missing parameter".to_string(),
            FaqError::DatabaseError(e) => format!("Database Error: {}", e),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
