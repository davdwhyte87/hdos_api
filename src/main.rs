use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::web::{Data, resource, route, service};


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
mod middlewares;





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

            .service(
                // all authenticated endpoints
                web::scope("api/v1/auth")
                    .service(user_controller::say_hello)
                    .wrap(middlewares::auth_middleware::AuthM)

                    // Test records routes
                    .service(test_record_controller::nurse_get_all_records)
                    .service(test_record_controller::nurse_create_test_record)
                    .service(test_record_controller::patient_get_all_records)
                    .service(test_record_controller::nurse_get_single_test_record)
                    .service(test_record_controller::patient_get_single_test_record)

                    // Test data routes
                    .service(test_data_controller::create_test_data)
                    .service(test_data_controller::update_test_data)

                    // Diagnosis routes
                    .service(diagnosis_controller::add_dignosis)
                    .service(diagnosis_controller::update_diagnosis)
                    .service(diagnosis_controller::nurse_get_diagnosis)
                    .service(diagnosis_controller::patient_get_diagnosis)
                    .service(diagnosis_controller::get_single_diagnosis)


            )
            .service(user_controller::create_user)
            .service(user_controller::login_user)

            //


    })
        .bind(("127.0.0.1", 80))?
        .run()
        .await
}