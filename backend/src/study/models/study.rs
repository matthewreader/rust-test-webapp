use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Study {
    pub study_id: i32,
    pub protocol_id: String,
    pub protocol_description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateStudy {
    pub protocol_id: String,
    pub protocol_description: String,
}

impl From<web::Json<CreateStudy>> for CreateStudy {
    fn from(new_study: web::Json<CreateStudy>) -> Self {
        CreateStudy {
            protocol_id: new_study.protocol_id.clone(),
            protocol_description: new_study.protocol_description.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateStudy {
    pub protocol_id: Option<String>,
    pub protocol_description: Option<String>
}

impl From<web::Json<UpdateStudy>> for UpdateStudy {
    fn from(update_study: web::Json<UpdateStudy>) -> Self {
        UpdateStudy {
            protocol_id: new_study.protocol_id.clone(),
            protocol_description: new_study.protocol_description.clone(),
        }
    }
}
