use crate::errors::StudyError;
use crate::models::study::*;
use sqlx::postgres::PgPool;

pub async fn post_new_study_db(
    pool: &PgPool,
    new_study: CreateStudy,
) -> Result<Study, StudyError> {
    let study_row= sqlx::query_as!(
        Study,
        "INSERT INTO study (protocol_id, protocol_description) 
        VALUES ($1,$2) 
        RETURNING study_id, protocol_id, protocol_description", 
        new_study.protocol_id, 
        new_study.protocol_description)
    .fetch_one(pool)
    .await?;

    Ok(study_row)
}

/*
pub async fn get_study_details_db(
    pool: &PgPool,
    study_id: i32,
) -> Result<Study, StudyError> {
    // Prepare SQL statement
    let study_row = sqlx::query_as!(
        Study,
        "SELECT study_id, protocol_id, protocol_description FROM study where study_id = $1",
        study_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(study) = study_row {
        Ok(study)
    } else {
        Err(StudyError::NotFound("Course id not found".into()))
    }
}
*/