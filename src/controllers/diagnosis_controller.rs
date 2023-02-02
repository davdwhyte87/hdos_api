use std::borrow::{Borrow, BorrowMut};
use actix_web::{get, HttpResponse, post, put};
use actix_web::web::{Data, Json, Path};
use mongodb::Database;
use validator::Validate;
use crate::models::diagnosis::Diagnosis;
use crate::models::request_models::{CreateDiagnosisReq, UpdateDiagnosisReq};
use crate::req_models::create_user_req::CreateUserReq;
use crate::services::diagnosis_service::DiagnosisService;
use crate::services::mongo_service::MongoService;


// a nurse should be abel to add a new diagnosis to a patient
#[post("/diagnosis/create")]
pub async fn add_dignosis(database:Data<MongoService>, new_diag:Json<CreateDiagnosisReq>) ->HttpResponse{

    let diagnosis = Diagnosis{
        id: None,
        note: new_diag.note.to_owned(),
        symptoms:new_diag.symptoms.to_owned(),
        created_at:  chrono::offset::Utc::now().to_string(),
        prescription: new_diag.prescription.to_owned(),
        updated_at:None,
        patient_email:new_diag.patient_email.to_owned(),
        nurse_email: new_diag.nurse_email.to_owned()
    };


    // validate request
    match new_diag.validate() {
        Ok(_)=>{},
        Err(err)=>{
            return HttpResponse::BadRequest().json(err);
        }
    }
    let res_dig = DiagnosisService::create(database.db.borrow(), diagnosis).await;
    match res_dig {
        Ok(dg)=>{
            HttpResponse::Ok().json(dg)
        }
        Err(err)=>{
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}


#[put("diagnosis/{id}")]
pub async fn update_diagnosis(database:Data<MongoService>, path:Path<String>, new_diag:Json<UpdateDiagnosisReq>)->HttpResponse{
    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };

    let mut get_diagnosis_result = DiagnosisService::get_by_id(database.db.borrow(), id.to_string()).await;
    let mut diagnosis = match  get_diagnosis_result{
        Ok(dignosis)=>{
            match dignosis {
                Some(mut diagnosis)=>{diagnosis},
                None=>{
                    return HttpResponse::NotFound().body("No data found");
                }
            }
        },
        Err(error)=>{
            return HttpResponse::BadRequest().body(error.to_string());
        }

    };

    // let mut  diagnosis = get_diagnosis_result.unwrap().unwrap();
    if (!new_diag.note.is_empty()){
        diagnosis.note = new_diag.note.to_owned();
    }
    if (!new_diag.symptoms.is_empty()){
        diagnosis.symptoms = new_diag.symptoms.to_owned();
    }
    if (!new_diag.prescription.is_empty()){
        diagnosis.prescription = new_diag.prescription.to_owned();
    }
    diagnosis.updated_at = Option::from(chrono::offset::Utc::now().to_string());

    let update_res = DiagnosisService::update(database.db.borrow(), id.to_string(), diagnosis.borrow_mut()).await;
    match update_res {
        Ok(_)=>{ HttpResponse::Ok().body("ok")},
        Err(err)=>{HttpResponse::InternalServerError().body(err.to_string())}
    }

}


#[get("/diagnosis/patient/{email}")]
pub async fn get_user_diagnosis(database:Data<MongoService>,
                                path:Path<String>)->HttpResponse{

    let email =path.into_inner();
    if email.is_empty(){
        return HttpResponse::BadRequest().body("Invalid email");
    };

    let diag_res = DiagnosisService::get_by_patient_email(database.db.borrow(), email.to_string()).await;
    match diag_res {
        Ok(diagnosis)=>{return HttpResponse::Ok().json(diagnosis)},
        Err(error)=>{return HttpResponse::InternalServerError().body(error.to_string())}
    }
}


#[get("/diagnosis/{id}")]
pub async fn get_single_diagnosis(database:Data<MongoService>,
                                path:Path<String>)->HttpResponse{

    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };

    let diag_res = DiagnosisService::get_by_id(database.db.borrow(), id.to_string()).await;
    match diag_res {
        Ok(diagnosis)=>{return HttpResponse::Ok().json(diagnosis)},
        Err(error)=>{return HttpResponse::InternalServerError().body(error.to_string())}
    }
}