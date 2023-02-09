use std::borrow::Borrow;
use actix_web::{get, HttpResponse, post};
use actix_web::web::{Data, Json, Path, ReqData};
// use serde::de::Unexpected::Option;
// use serde::de::Unexpected::Option;
use serde::Serialize;
use validator::Validate;
use crate::models::request_models::CreateTestRecordReq;
use crate::models::response::{Response, ResponseInsert};
use crate::models::test_record::TestRecord;
use crate::models::user::{User, UserType};
use crate::services::mongo_service::MongoService;
use crate::services::test_record_service::TestRecordService;
use crate::utils::auth::Claims;


#[post("test_record/create")]
pub async fn nurse_create_test_record(database:Data<MongoService>,
                                      data: Json<CreateTestRecordReq>,
                                      claim:Option<ReqData<Claims>>)
    ->HttpResponse
{


    // validate request
    match data.validate() {
        Ok(_)=>{},
        Err(err)=>{
            return HttpResponse::BadRequest().json(err);
        }
    }

    // get claim data for auth from req
    if let Some(claim) = claim{
        print!("{:?}", claim);
        // return error message if the user is not a nurse
        if !(claim.role == UserType::Nurse || claim.role == UserType::Hospital){
            return HttpResponse::Unauthorized()
                .json(Response{message:"You do not have permission".to_string()})
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
pub async fn nurse_get_all_records(database:Data<MongoService>,
                                   claim:Option<ReqData<Claims>>)->HttpResponse{

    // get claim data for auth from req
    if let Some(claim) = claim{
        print!("{:?}", claim);
        // return error message if the user is not a nurse
        if !(claim.role == UserType::Nurse || claim.role == UserType::Hospital){
            return HttpResponse::Unauthorized()
                .json(Response{message:"You do not have permission".to_string()})
        }
    }
    let result =TestRecordService::get_all_records(database.db.borrow()).await;
    match result {
        Ok(res)=>{return HttpResponse::Ok().json(res)},
        Err(err)=>{
            return HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}


// user get his own test record
#[get("/test_record/patient/all")]
pub async fn patient_get_all_records(database:Data<MongoService>,
                                   claim:Option<ReqData<Claims>>
)->HttpResponse{

    // get claim data for auth from req
    let claim_data = match claim {
        Some(claim_data)=>{claim_data},
        None=>{
            return HttpResponse::Unauthorized()
            .json(Response{message:"You do not have permission".to_string()})
        }
    };

    // check if the logged in user is a patient
    if !(claim_data.role == UserType::Patient){
        return HttpResponse::Unauthorized()
            .json(Response{message:"You do not have permission".to_string()})
    }
    let result =TestRecordService::get_by_patient_email(database.db.borrow(), claim_data.email.to_string()).await;
    match result {
        Ok(res)=>{return HttpResponse::Ok().json(res)},
        Err(err)=>{
            return HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}


// get single test record (hospital, nurse)
#[get("/test_record/{id}")]
pub async fn nurse_get_single_test_record(
    database:Data<MongoService>,
    path:Path<String>,
    claim:Option<ReqData<Claims>>
)->HttpResponse{
    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };

    //check if user is authorized
    let claim_data = match claim {
        Some(claim_data)=>{claim_data},
        None=>{
            return HttpResponse::Unauthorized().body("Unauthorised, no claims".to_string())
        }
    };
    if !(claim_data.role ==UserType::Nurse || claim_data.role==UserType::Hospital){
        return HttpResponse::Unauthorized().json(Response{
            message:"You do not have permission for this resource".to_string()
        });
    }

    // get data from the db
    let test_record = TestRecordService::get_by_id(
        database.db.borrow(), id).await;
    let test_record = match test_record {
        Ok(test_record)=>{
            match test_record {
                Some(test_record)=>{test_record},
                None=>{
                    return HttpResponse::BadRequest().json(Response{
                        message:"There is no test record".to_string()})
                }
            }
        },
        Err(err)=>{
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    };

    return HttpResponse::Ok().json(test_record)
}


// get single test record (hospital, nurse)
#[get("/test_record/patient/{id}")]
pub async fn patient_get_single_test_record(
    database:Data<MongoService>,
    path:Path<String>,
    claim:Option<ReqData<Claims>>
)->HttpResponse{
    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };

    //check if user is authorized
    let claim_data = match claim {
        Some(claim_data)=>{claim_data},
        None=>{
            return HttpResponse::Unauthorized().body("Unauthorised, no claims".to_string())
        }
    };
    if (claim_data.role !=UserType::Patient){
        return HttpResponse::Unauthorized().json(Response{
            message:"You do not have permission for this resource".to_string()
        });
    }

    // get data from the db
    let test_record = TestRecordService::get_by_id(
        database.db.borrow(), id).await;
    let test_record = match test_record {
        Ok(test_record)=>{
            match test_record {
                Some(test_record)=>{test_record},
                None=>{
                    return HttpResponse::BadRequest().json(Response{
                        message:"There is no test record".to_string()})
                }
            }
        },
        Err(err)=>{
            return HttpResponse::InternalServerError().json(err.to_string())
        }
    };

    // check if the patient owns the test record
    if(test_record.patient_email != claim_data.email){
        return HttpResponse::Unauthorized().json(Response{
            message:"You do not have permission for this resource".to_string()
        });
    }


    // return value
    return HttpResponse::Ok().json(test_record)
}
