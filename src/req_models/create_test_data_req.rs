
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTestDataReq{
    pub name:String,
    pub result:String,
    pub test_record_id:String,
    pub nurse_email:String
}