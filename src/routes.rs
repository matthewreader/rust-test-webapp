use crate::handlers::{general::*, study::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn study_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/study")
        .service(post_new_study)
        .service(get_study_by_id);
    cfg.service(scope);
}
