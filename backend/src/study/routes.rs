use crate::handlers::{study::*, general::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn study_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/study")
            .route("/", web::post().to(post_new_study))
           //.route("/{study_id}", web::get().to(get_study_details))
    );
}