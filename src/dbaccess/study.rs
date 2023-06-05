use crate::errors::StudyError;
use crate::models::study::*;
use actix_web::HttpResponse;
use serde_json::json;
use sqlx::postgres::PgPool;

pub async fn post_new_study_db(pool: &PgPool, new_study: CreateStudy) -> HttpResponse {
    let query_result = sqlx::query_as!(
        Study,
        "INSERT INTO study (protocol_id, protocol_description) 
        VALUES ($1,$2) 
        RETURNING study_id, protocol_id, protocol_description",
        new_study.protocol_id,
        new_study.protocol_description
    )
    .fetch_one(pool)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    return match query_result {
        Ok(study) => {
            let study_response = json!({"status": "success","data": serde_json::json!({
                "study": &study
            })});

            HttpResponse::Ok().json(study_response)
        }

        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": format!("{:?}", e)})),
    };
}

pub async fn get_study_by_id_db(pool: &PgPool, id: i32) -> HttpResponse {
    let query_result = sqlx::query_as!(
        Study,
        "SELECT *
        FROM study
        WHERE study_id = $1",
        id
    )
    .fetch_one(pool)
    .await;

    return match query_result {
        Ok(study) => {
            let study_response = json!({"status": "success","data": serde_json::json!({
                "study": &study
            })});

            HttpResponse::Ok().json(study_response)
        }
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .json(json!({"status": "fail","message": format!("Study with ID: {} not found", id)})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": format!("{:?}", e)})),
    };
}
