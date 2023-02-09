use std::borrow::{Borrow, BorrowMut};
use actix_web::{Responder, get, HttpResponse, web::Json, post};
use actix_web::web::{Data, ReqData};
use handlebars::Handlebars;
use validator::Validate;
use crate::database::db::db::DB;
use crate::models::helper::EmailData;
use crate::models::request_models::LoginReq;
use crate::models::response::{LoginResp, Response};
use crate::models::user::User;
use crate::req_models::create_user_req::CreateUserReq;
use crate::services::mongo_service::MongoService;
use crate::services::user_service::UserService;
use crate::utils::auth::{Claims, decode_token, encode_token};
use crate::utils::send_email::send_email;


#[get("/say_hello")]
pub async fn say_hello(claim:Option<ReqData<Claims>>)-> HttpResponse{
    print!("Hello maaa gee");
    if let Some(claim) = claim{
        print!("{:?}", claim)
    }

    print!("Hello maaa gee");
    let response = Response{
        message:"good".to_string(),
    };
    // match DB::initialize_db().await{
    //     Ok((_))=>{},
    //     Err(err)=>{println!("{:?}", err)}
    // }
    // let tdata = decode_token("\
    // eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJyb2xlIjoiUGF0aWVudCIsImVtYWlsIjoicGF0aWVudDFAeC5jb20iLCJuYW1lIjoicGF0aWVudDEiLCJleHAiOjE2NzU1NjI1ODB9.lSUV9_cvLqYXgsvfvbbr5s_QqDtFzbIux6ePVSKu9xo\
    // ".to_string());
    // let tdata = match tdata {
    //     Ok(tdata)=>{tdata},
    //     Err(err)=>{return HttpResponse::InternalServerError().json(err.to_string())}
    // };
    // println!("{:?}",  tdata);
    return HttpResponse::Created().json(response)
}

#[post("/user")]
pub async fn create_user(database:Data<MongoService>, new_user:Json<CreateUserReq>)->HttpResponse{
    let user = User{
        name:new_user.name.to_owned(),
        created_at:chrono::offset::Utc::now().to_string(),
        email:new_user.email.to_owned(),
        code:Option::from(93030),
        user_type: new_user.into_inner().user_type,
        id:None
    };
    let user_res = UserService::create_user(database.db.borrow(),&user).await;
    match user_res {
        Ok(user)=>HttpResponse::Ok().json(user),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}


#[post("/user/login")]
pub async fn login_user(database:Data<MongoService>, req_data:Json<LoginReq>)->HttpResponse{
    //validate request data
    {
        match req_data.borrow().validate() {
            Ok(_)=>{},
            Err(err)=>{
                return HttpResponse::BadRequest().json(err);
            }
        }
    }

    // convert code to int
    let code = req_data.code.parse::<i32>();
    let code = match code {
        Ok(code)=>{code},
        Err(err)=>{return return HttpResponse::BadRequest().
            json(Response{message:"Error  getting string".to_string()})}
    };

    // check if the user sent the right otp
    // get user data from db
    let get_user_res = UserService::get_by_email(
        database.db.borrow(), req_data.borrow().email.to_owned()).await;
    let user = match  get_user_res{
        Ok(user)=>{
            match user {
                Some(user)=>{user},
                None=>{return return HttpResponse::InternalServerError().
                    json(Response{message:"User Not Found".to_string()})}
            }
        },
        Err(err)=>{
            // log error
            return HttpResponse::InternalServerError().
            json(Response{message:"Error getting user".to_string()})}
    };
    let real_code = match user.code{
        Some(real_code)=>{real_code},
        None=>{
            return HttpResponse::BadRequest().
                json(Response{message:"Get auth code".to_string()})
        }
    };
    //check if user has the right code
    if (real_code !=  code){
        return HttpResponse::Unauthorized().
            json(Response{message:"Wrong auth data".to_string()})
    }
    //if he has the right code send email
    {
        send_new_login_email(user.borrow(), chrono::offset::Utc::now().to_string());
    }

    // send token
    let login_token =encode_token(
        user.user_type, req_data.borrow().email.as_str().to_string(),user.name);
    let login_token = match login_token {
        Ok(login_token)=>{login_token},
        Err(err)=>{
            return HttpResponse::InternalServerError().
                json(Response{message:"Error getting token".to_string()})
        }
    };

    HttpResponse::Ok()
        .json(LoginResp{message:"Logged in".to_string(), token:login_token})
}

async fn send_new_login_email(user:&User, time:String){
    let name = user.name.as_str().to_string();

    let mut reg = Handlebars::new();
    let order_email_content = reg.render_template (
        include_str!("../utils/html/new_login.hbs"),
        &serde_json::json!({"name" :name, "time":time})).unwrap();

    let email_data = EmailData{
        subject:"New Login".to_string(),
        to: (*user.email).parse().unwrap(),
        body: order_email_content
    };
    send_email(email_data);
}
