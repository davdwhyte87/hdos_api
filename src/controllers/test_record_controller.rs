use std::borrow::Borrow;
use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
use crate::models::request_models::CreateTestRecordReq;
use crate::models::test_record::TestRecord;
use crate::services::mongo_service::MongoService;
use crate::services::test_record_service::TestRecordService;


#[post("test_record/create")]
pub async fn create_test_record(database:Data<MongoService>, data: Json<CreateTestRecordReq>) ->HttpResponse{
  let test_record = TestRecord{
      nurse_email:data.nurse_email.to_owned(),
      id:None,
      diagnosis_id:None,
      note:data.note.to_owned(),
      patient_email:data.patient_email.to_owned(),
      test_datas: Vec::new(),
      created_at:chrono::offset::Utc::now().to_string(),
  }  ;
  let insert_result =TestRecordService::create(database.db.borrow(), test_record).await;
    match insert_result {
        Ok(_)=>{return HttpResponse::Ok().body("Ok")},
        Err(err)=>{return HttpResponse::InternalServerError().body(err.to_string())}
    }
}