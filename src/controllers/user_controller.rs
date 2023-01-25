use std::borrow::Borrow;
use actix_web::{Responder, get, HttpResponse, web::Json, post};
use actix_web::web::Data;
use crate::database::db::db::DB;
use crate::models::response::Response;
use crate::models::user::User;
use crate::services::mongo_service::MongoService;
use crate::services::user_service::UserService;

#[get("/say_hello")]
pub async fn say_hello()-> HttpResponse{
    format!("Hello maaa gee");
    let response = Response{
        message:"good".to_string(),
    };
    match DB::initialize_db().await{
        Ok((_))=>{},
        Err(err)=>{println!("{:?}", err)}
    }
    return HttpResponse::Created().json(response)
}

#[post("/user")]
pub async fn create_user(database:Data<MongoService>, new_user:Json<User>)->HttpResponse{
    let user = User{
        name:new_user.name.to_owned(),
        id:None
    };
    let user_res = UserService::create_user(database.db.borrow(),user).await;
    match user_res {
        Ok(user)=>HttpResponse::Ok().json(user),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}
