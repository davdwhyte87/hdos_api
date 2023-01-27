use mongodb::bson::oid::ObjectId;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Diagnosis {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub symptoms:String,
    pub created_at: String,
    pub prescription: String,
    pub note: String,
    pub nurse_id:String
}