use crate::dbaccess::study::*;
use crate::models::study::CreateStudy;
use crate::state::AppState;
use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::Error;

#[post("/new")]
pub async fn post_new_study(
    new_study: web::Json<CreateStudy>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    post_new_study_db(&app_state.db, new_study.into()).await
}

#[get("/{id}")]
pub async fn get_study_by_id(app_state: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    get_study_by_id_db(&app_state.db, id).await
}
