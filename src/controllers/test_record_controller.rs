use std::borrow::Borrow;
use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
// use serde::de::Unexpected::Option;
// use serde::de::Unexpected::Option;
use serde::Serialize;
use validator::Validate;
use crate::models::request_models::CreateTestRecordReq;
use crate::models::response::{Response, ResponseInsert};
use crate::models::test_record::TestRecord;
use crate::services::mongo_service::MongoService;
use crate::services::test_record_service::TestRecordService;


#[post("test_record/create")]
pub async fn create_test_record(database:Data<MongoService>, data: Json<CreateTestRecordReq>) ->HttpResponse{
    // validate request
    match data.validate() {
        Ok(_)=>{},
        Err(err)=>{
            return HttpResponse::BadRequest().json(err);
        }
    }
    let test_record = TestRecord{
      nurse_email:data.nurse_email.to_owned(),
      id:None,
      diagnosis_id:None,
      note:data.note.to_owned(),
      patient_email:data.patient_email.to_owned(),
      test_datas: Vec::new(),
      created_at:chrono::offset::Utc::now().to_string(),
      test_data: Option::from(Vec::new())
  };


  let insert_result =TestRecordService::create(database.db.borrow(), test_record).await;
    match insert_result {
        Ok(insert_data)=>{
            return HttpResponse::Ok().json(ResponseInsert{
                message:"ok".to_string(),
                data:insert_data
            })
        },
        Err(err)=>{return HttpResponse::InternalServerError().body(err.to_string())}
    }
}

#[get("/test_record/all")]
pub async fn nurse_all_records(database:Data<MongoService>)->HttpResponse{
    let result =TestRecordService::get_all_records(database.db.borrow()).await;
    match result {
        Ok(res)=>{return HttpResponse::Ok().json(res)},
        Err(err)=>{
            return HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}