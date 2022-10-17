use crate::models::study::*;
use sqlx::postgres::PgPool;

pub async fn post_new_study_db(
    pool: &PgPool,
    new_course: CreateStudy,
) -> Result<Course, E> {
    let study_row= sqlx::query_as!(Course,"insert into study (study_id, protocol_id, protocol_description ($1,$2,$3) returning study_id, protocol_id, protocol_description", 
    new_study.study_id, new_study.protocol_id, new_study.protocol_description)
    .fetch_one(pool)
    .await?;
    Ok(course_row)
}

pub async fn get_study_details_db(
    pool: &PgPool,
    study_id: i32,
) -> Result<Course, E> {
    // Prepare SQL statement
    let study_row = sqlx::query_as!(
        Study,
        "SELECT * FROM study where study_id = $1",
        study_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(study) = course_row {
        Ok(course)
    } else {
        Err(EzyTutorError::NotFound("Course id not found".into()))
    }
}