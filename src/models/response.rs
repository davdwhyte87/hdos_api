use mongodb::results::InsertOneResult;
use serde::Serialize;



#[derive(Serialize)]
pub struct Response {
    pub message: String,

}

#[derive(Serialize)]
pub struct ResponseInsert{
    pub message: String,
    pub data: InsertOneResult
}

#[derive(Serialize)]
pub struct LoginResp{
    pub message: String,
    pub token: String
}
