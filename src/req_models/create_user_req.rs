
use serde_derive::{Deserialize, Serialize};
use crate::models::user::UserType;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserReq{
    pub name:String,
    pub email:String,
    pub user_type:UserType
}