use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum StudyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for StudyError {}

impl StudyError {
    fn error_response(&self) -> String {
        match self {
            StudyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            StudyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            StudyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            StudyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for StudyError {
    fn status_code(&self) -> StatusCode {
        match self {
            StudyError::DBError(_msg) | StudyError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            StudyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            StudyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for StudyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for StudyError {
    fn from(err: actix_web::error::Error) -> Self {
        StudyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for StudyError {
    fn from(err: SQLxError) -> Self {
        StudyError::DBError(err.to_string())
    }
}