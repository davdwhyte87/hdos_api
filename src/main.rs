use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::web::Data;


mod controllers;
use controllers::user_controller::{create_user, say_hello};
mod models;
use models::{response};
mod database;
use database::db::db;
mod services;
use services::{user_service, pet_service};
use crate::services::mongo_service::MongoService;
mod utils;




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
            .service(say_hello)
            .service(create_user)
    
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}