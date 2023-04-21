use crate::dbaccess::study::*;
use crate::errors::StudyError;
use crate::models::study::CreateStudy;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn post_new_study(
    new_study: web::Json<CreateStudy>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, StudyError> {
    post_new_study_db(&app_state.db, new_study.into())
        .await
        .map(|study| HttpResponse::Ok().json(study))
}
/*
pub async fn get_study_details(
    app_state: web::Data<AppState>,
    web::Path(study_id): web::Path<i32>,
) -> Result<HttpResponse, StudyError> {
    get_study_details_db(&app_state.db, study_id)
        .await
        .map(|study| HttpResponse::Ok().json(study))
}
*/
