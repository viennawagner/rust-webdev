use crate::*;

#[derive(Debug)]
enum FaqError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}
impl std::fmt::Display for FaqError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FaqError::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            },
            FaqError::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}
impl IntoResponse for FaqError {
    fn into_response(self) -> Response {
        let body = match self {
            FaqError::ParseError(ref err) => {
                format!("Cannot parse parameter: {}", err)
            },
            FaqError::MissingParameters => "Missing parameter".to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}