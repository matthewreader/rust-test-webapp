use crate::errors::StudyError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub async fn not_found() -> Result<HttpResponse, StudyError> {
    let response = "{error: not found}";
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).json(&response))
}
