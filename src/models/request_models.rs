
use mongodb::bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct  CreateDiagnosisReq{
    pub symptoms:String,
    pub prescription: String,
    pub note: String,
    pub patient_email:String,
    pub nurse_email:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  UpdateDiagnosisReq{
    pub symptoms:String,
    pub prescription: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  CreateTestRecordReq{
    pub nurse_email:String,
    pub patient_email: String,
    pub note:String
}