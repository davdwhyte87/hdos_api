

use std::borrow::Borrow;
use actix_web::{get, HttpResponse, post, put};
use actix_web::web::{Data, Json, Path};
use mongodb::bson::oid::ObjectId;

use crate::models::request_models::UpdateTestDataReq;
use crate::models::response::Response;
use crate::models::test_data::TestData;
use crate::models::test_record::TestRecord;
use crate::req_models::create_test_data_req::CreateTestDataReq;
use crate::services::mongo_service::MongoService;
use crate::services::test_data_service::TestDataService;
use crate::services::test_record_service::TestRecordService;


#[post("test_data/create")]
pub async fn create_test_data(database:Data<MongoService>, req_data :Json<CreateTestDataReq>) ->HttpResponse{
    let test_data_record_id = req_data.test_record_id.clone();
    // get test record data
    let get_trecord_result = TestRecordService::get_by_id(database.db.borrow(),test_data_record_id).await;
    let mut test_record = match get_trecord_result {
        Ok(test_record)=>{
            match test_record {
                Some(test_record)=>{test_record},
                None=>{return HttpResponse::BadRequest().body("Test Record does not exist ")}
            }
        },
        Err(err)=>{
            return HttpResponse::InternalServerError().body(err.to_string())
        }
    };

    // set new test data values
    let test_data = TestData{
        nurse_email:req_data.nurse_email.to_owned(),
        id:None,
        created_at:chrono::offset::Utc::now().to_string(),
        updated_at:chrono::offset::Utc::now().to_string(),
        test_record_id: match ObjectId::parse_str(req_data.test_record_id.to_owned()) {
            Ok(test_record_id) => {test_record_id}
            Err(err) => {return HttpResponse::InternalServerError().body(err.to_string())}
        },
        name: req_data.name.to_owned(),
        result: req_data.result.to_owned()
    }  ;
    let test_data_id =TestDataService::create(database.db.borrow(), test_data.borrow()).await;

    let test_data_id = match test_data_id {
        Ok(test_data_id)=>{ match test_data_id.inserted_id.as_object_id() {
            Some(test_data_id) => {test_data_id},
            None => {return HttpResponse::InternalServerError().body("Error getting test data id decoded")}

        } },
        Err(err)=>{return HttpResponse::InternalServerError().body(err.to_string())}
    };

    // update test record with new test data id

    // let test_data_id = match test_data.id {
    //     Some(test_data_id) => {test_data_id},
    //     None => {return HttpResponse::InternalServerError().body("cannot get test data ID") }
    // };
   let mut test_datas = test_record.test_datas;
    test_datas.push(test_data_id);

    test_record.test_datas = test_datas;

    let update_res = TestRecordService::update(database.db.borrow(), test_record.id.unwrap().to_string(), test_record.borrow()).await;
    match update_res {
        Ok(_)=>{ return HttpResponse::Ok().json(Response{message:"Test Data created".to_string()})},
        Err(err)=>{
            return HttpResponse::InternalServerError().body(err.to_string())
        }
    }
    // return HttpResponse::Ok().body("")
}


#[put("/test_data/{id}")]
pub async fn update_test_data(database:Data<MongoService>, path:Path<String>, req_data:Json<UpdateTestDataReq>)->HttpResponse{
    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };
    let test_data_res = TestDataService::get_by_id(
        database.db.borrow(),id.to_string() ).await;
    let testdata = match test_data_res {
        Ok(test_data)=>{test_data},
        Err(err)=>{return HttpResponse::InternalServerError().json(Response{message:err.to_string(), })}
    };
    let mut testdata = match testdata{
        Some(testdata)=>{testdata},
        None=>{return HttpResponse::NotFound().json(Response{message:"Test data not found".to_string(),})}
    };

    if(!req_data.name.is_empty()){
       testdata.name = req_data.name.to_owned();
    }
    if(!req_data.result.is_empty()){
        testdata.result = req_data.result.to_owned();
    }

    let update_res = TestDataService::update(
        database.db.borrow(), id.to_string(), testdata.borrow()).await;
    match update_res {
        Ok(result)=>{return HttpResponse::Ok().json(result)},
        Err(err)=>{
            return HttpResponse::InternalServerError().json(Response{message:err.to_string(),})
        }
    }

    // return HttpResponse::Ok().body("OK")
}