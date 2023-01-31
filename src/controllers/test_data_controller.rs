

use std::borrow::Borrow;
use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json};
use crate::models::test_data::TestData;
use crate::models::test_record::TestRecord;
use crate::req_models::create_test_data_req::CreateTestDataReq;
use crate::services::mongo_service::MongoService;
use crate::services::test_data_service::TestDataService;
use crate::services::test_record_service::TestRecordService;


#[post("test_data/create")]
pub async fn create_test_data(database:Data<MongoService>, req_data :Json<CreateTestDataReq>) ->HttpResponse{
    let test_data = TestData{
        nurse_email:req_data.nurse_email.to_owned(),
        id:None,
        created_at:chrono::offset::Utc::now().to_string(),
        updated_at:chrono::offset::Utc::now().to_string(),
        test_record_id: req_data.test_record_id.to_owned(),
        name: req_data.name.to_owned(),
        result: req_data.result.to_owned()
    }  ;
    let insert_result =TestDataService::create(database.db.borrow(), test_data).await;
    match insert_result {
        Ok(_)=>{return HttpResponse::Ok().body("Ok")},
        Err(err)=>{return HttpResponse::InternalServerError().body(err.to_string())}
    }
}