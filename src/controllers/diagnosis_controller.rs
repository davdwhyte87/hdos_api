use std::borrow::{Borrow, BorrowMut};
use actix_web::{get, HttpResponse, post, put};
use actix_web::web::{Data, Json, Path, ReqData};
use mongodb::Database;
use validator::Validate;
use crate::models::diagnosis::Diagnosis;
use crate::models::request_models::{CreateDiagnosisReq, UpdateDiagnosisReq};
use crate::models::response::Response;
use crate::models::user::{User, UserType};
use crate::req_models::create_user_req::CreateUserReq;
use crate::services::diagnosis_service::DiagnosisService;
use crate::services::mongo_service::MongoService;
use crate::utils::auth::Claims;


// a nurse should be abel to add a new diagnosis to a patient
#[post("/diagnosis/create")]
pub async fn add_dignosis(database:Data<MongoService>,
                          new_diag:Json<CreateDiagnosisReq>,
                          claim:Option<ReqData<Claims>>
) ->HttpResponse{

    //
    match new_diag.validate() {
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
pub async fn update_diagnosis(database:Data<MongoService>,
                              path:Path<String>,
                              new_diag:Json<UpdateDiagnosisReq>,
                              claim:Option<ReqData<Claims>>
)->HttpResponse{


    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };

    //validate request data
    match new_diag.validate() {
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


// this allows a nurse get all  diagnosis
#[get("/diagnosis/patient/{email}")]
pub async fn nurse_get_diagnosis(database:Data<MongoService>,
                                      path:Path<String>,
                                      claim:Option<ReqData<Claims>>
)->HttpResponse{
    let email =path.into_inner();
    if email.is_empty(){
        return HttpResponse::BadRequest().body("Invalid email");
    };

    let claim_data = match claim {
        Some(claim_data)=>{claim_data},
        None=>{
            return HttpResponse::Unauthorized().body("Unauthorised, no claims".to_string())
        }
    };
    // validate that we have a patient or hospital
    // return error message if the user is not a nurse
    if !(claim_data.role == UserType::Nurse || claim_data.role == UserType::Hospital){
        // get all the diagnosis for the user email

    }

    let diag_res = DiagnosisService::get_by_patient_email(database.db.borrow(), email.to_string()).await;
    match diag_res {
        Ok(diagnosis)=>{return HttpResponse::Ok().json(diagnosis)},
        Err(error)=>{return HttpResponse::InternalServerError().body(error.to_string())}
    }
}



// this allows a patient get his dignosis data
#[get("/diagnosis/patient")]
pub async fn patient_get_diagnosis(database:Data<MongoService>,
                                claim:Option<ReqData<Claims>>
)->HttpResponse{

    let claim_data = match claim {
        Some(claim_data)=>{claim_data},
        None=>{
            return HttpResponse::Unauthorized().body("Unauthorised, no claims".to_string())
        }
    };
    // validate that we have a patient or hospital
    // return error message if the user is not a nurse
    if !(claim_data.role == UserType::Patient){
        // get all the diagnosis for the user email
        return HttpResponse::Unauthorized().body("Unauthorised, no claims".to_string())
    }

    let diag_res = DiagnosisService::get_by_patient_email(
        database.db.borrow(),
        claim_data.email.to_string()).await;
    match diag_res {
        Ok(diagnosis)=>{return HttpResponse::Ok().json(diagnosis)},
        Err(error)=>{return HttpResponse::InternalServerError().body(error.to_string())}
    }
}






#[get("/diagnosis/{id}")]
pub async fn get_single_diagnosis(database:Data<MongoService>,
                                  path:Path<String>,
                                  claim:Option<ReqData<Claims>>
)->HttpResponse{

    let id =path.into_inner();
    if id.is_empty(){
        return HttpResponse::BadRequest().body("Invalid id");
    };

    let claim_data = match claim {
        Some(claim_data)=>{claim_data},
        None=>{
            return HttpResponse::Unauthorized().body("Unauthorised, no claims".to_string())
        }
    };

    let diag_res = DiagnosisService::get_by_id(database.db.borrow(), id.to_string()).await;
    match diag_res {
        Ok(diagnosis)=>{
            // if the user is a patient then check if the
            // diagnosis is for the user
            if (claim_data.role==UserType::Patient){
                // get the data for the logged in user
                match diagnosis.borrow(){
                    Some(diagnosis)=>{
                        if diagnosis.patient_email == claim_data.email{
                            // the user is a patient and the diagnosis we got belongs to him
                            return HttpResponse::Ok().json(diagnosis)
                        }
                    },
                    None=>{
                        return HttpResponse::Unauthorized().json(Response{
                            message:"You do not have permission for this resource".to_string()
                        })
                    }
                };

            }
            return HttpResponse::Ok().json(diagnosis)
        },
        Err(error)=>{return HttpResponse::InternalServerError().body(error.to_string())}
    }
}