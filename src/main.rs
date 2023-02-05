use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::web::Data;


mod controllers;
use controllers::{user_controller, diagnosis_controller, test_data_controller, test_record_controller};
mod models;
use models::{response};
mod database;
use database::db::db;
mod services;
use services::{user_service, pet_service, diagnosis_service};
use crate::services::mongo_service::MongoService;
mod utils;
mod req_models;





#[get("/")]
async fn index() -> impl Responder {
    "Hello, Bread!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    let db = MongoService::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move|| {
        App::new()
            .app_data(db_data.clone())

            // USER CONTROLLERS
            .service(user_controller::say_hello)
            .service(user_controller::create_user)
            .service(user_controller::login_user)

            //
            .service(diagnosis_controller::add_dignosis)
            .service(diagnosis_controller::update_diagnosis)
            .service(diagnosis_controller::get_user_diagnosis)
            .service(diagnosis_controller::get_single_diagnosis)
            .service(test_record_controller::create_test_record)
            .service(test_data_controller::create_test_data)
            .service(test_data_controller::update_test_data)
            .service(test_record_controller::nurse_all_records)
    })
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}