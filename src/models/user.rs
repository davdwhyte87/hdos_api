use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email:String,
    pub password:String,
    pub created_at:String,
    pub user_type:UserType
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserType{
    Patient,
    Nurse,
    Hospital
}