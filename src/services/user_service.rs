use std::string::ToString;
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use crate::database::db::db::DB;
use crate::models::user::User;

const COLLECTION_NAME:&str = "User";

pub struct UserService{
    client: Client

}

impl UserService{
    pub async fn create_user(db:&Database, user:User)->Result<InsertOneResult, Error>{
        // Get a handle to a collection in the database.
        let collection = db.collection::<User>(COLLECTION_NAME);

        // let new_user = User{
        //     id:None,
        //     name:user.name
        // };

        // Insert data into db.
        let res_user =collection.insert_one(user, None).await.ok().expect("Error creating user");
        Ok(res_user)
    }
}