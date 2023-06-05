use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "./dbaccess/mod.rs"]
mod dbaccess;
#[path = "./errors.rs"]
mod errors;
#[path = "./handlers/mod.rs"]
mod handlers;
#[path = "./models/mod.rs"]
mod models;
#[path = "./routes.rs"]
mod routes;
#[path = "state.rs"]
mod state;

use errors::StudyError;
use handlers::not_found::not_found;
use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    //Construct app and configure routes
    let app = move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                StudyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(study_routes)
            .default_service(web::route().to(not_found))
    };

    //Start HTTP server
    let host_port = dotenv::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
