
use mongodb::bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  CreateDiagnosisReq{
    #[validate(length(min=0))]
    pub symptoms:String,
    pub prescription: String,
    pub note: String,
    #[validate(email)]
    pub patient_email:String,
    #[validate(email)]
    pub nurse_email:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  UpdateDiagnosisReq{
    pub symptoms:String,
    pub prescription: String,
    pub note: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct  CreateTestRecordReq{
    #[validate(email)]
    pub nurse_email:String,
    #[validate(email)]
    pub patient_email: String,
    pub note:String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  UpdateTestDataReq{
    pub name:String,
    pub result: String,
}