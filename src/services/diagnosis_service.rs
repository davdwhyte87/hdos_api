
use std::fs::File;
use std::io::Read;
use std::string::ToString;
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;

use crate::database::db::db::DB;
use crate::models::diagnosis::Diagnosis;
use crate::models::helper::EmailData;
use crate::models::user::User;

use crate::utils::send_email::{ACTIVATE_EMAIL, send_email};

const COLLECTION_NAME:&str = "Diagnosis";

pub struct DiagnosisService{
    client: Client

}

impl DiagnosisService {
    pub async fn create(db: &Database, diagnosis: Diagnosis) -> Result<InsertOneResult, Error> {
        // Get a handle to a collection in the database.
        let collection = db.collection::<Diagnosis>(COLLECTION_NAME);
        let res_diag =collection.insert_one(diagnosis, None).await.ok().expect("Error creating diagnosis");
        Ok(res_diag)
    }

    pub async fn get_by_id(db:&Database, id:String)->Result<Diagnosis, Error>{
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":object_id};
        let collection = db.collection::<Diagnosis>(COLLECTION_NAME);
        let user_detail = collection.find_one(filter, None).await.ok().expect("Error getting diagnosis");
        Ok(user_detail.unwrap())
    }
}