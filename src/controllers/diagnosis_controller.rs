use std::borrow::Borrow;
use actix_web::{HttpResponse, post};
use actix_web::web::{Data, Json};
use crate::models::diagnosis::Diagnosis;
use crate::services::diagnosis_service::DiagnosisService;
use crate::services::mongo_service::MongoService;


// a nurse should be abel to add a new diagnosis to a patient
#[post("/diagnosis/add")]
pub async fn add_dignosis(database:Data<MongoService>, new_diag:Json<Diagnosis>) ->HttpResponse{
    let diagnosis = Diagnosis{
        id: None,
        note: new_diag.note.to_owned(),
        symptoms:new_diag.symptoms.to_owned(),
        created_at:  chrono::offset::Utc::now().to_string(),
        prescription: new_diag.prescription.to_owned(),
        nurse_id: new_diag.nurse_id.to_owned()
    };
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