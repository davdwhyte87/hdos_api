
use std::fs::File;
use std::io::Read;
use std::string::ToString;
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::{bson, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::extjson::de::Error;
use mongodb::results::{InsertOneResult, UpdateResult};
use futures::stream::TryStreamExt;
use mongodb::bson::extjson::de::Error::DeserializationError;
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

    pub async fn get_by_id(db:&Database, id:String)->Result<Option<Diagnosis>, Error>{
        let object_id = ObjectId::parse_str(id);
        let object_id = match object_id {
            Ok(object_id)=>{object_id},
            Err(error)=>{
                return Err(DeserializationError {message:"Error decoding id".to_string()})
            }
        };
        let filter = doc! {"_id":object_id};
        let collection = db.collection::<Diagnosis>(COLLECTION_NAME);
        let user_detail = collection.find_one(filter, None).await.ok().expect("Error getting diagnosis");
        Ok(user_detail)
    }

    pub async fn get_by_user_id(db:&Database, id:String)->Result<Vec<Diagnosis>, Error>{
        let object_id = ObjectId::parse_str(id);
        let object_id = match object_id {
            Ok(object_id)=>{object_id},
            Err(error)=>{
               return Err(DeserializationError {message:"Error decoding id".to_string()})
            }
        };
        let filter = doc! {"patient_id":object_id};
        let collection = db.collection::<Diagnosis>(COLLECTION_NAME);
        let mut cursor = collection.find(filter, None).await.ok().expect("Error getting diagnosis");
        let mut diagnosis:Vec<Diagnosis> = Vec::new();

        while let Some(diag)= cursor.try_next().await.ok().expect("Error matching "){
            diagnosis.push(diag);
        }
        Ok(diagnosis)
    }

    pub async fn get_by_patient_email(db:&Database, email:String)->Result<Vec<Diagnosis>, Error>{
        if email.is_empty(){
            return Err(DeserializationError {message:"Error decoding id".to_string()})
        }
        let filter = doc! {"patient_email":email};
        let collection = db.collection::<Diagnosis>(COLLECTION_NAME);
        let mut cursor = collection.find(filter, None).await.ok().expect("Error getting diagnosis");
        let mut diagnosis:Vec<Diagnosis> = Vec::new();

        while let Some(diag)= cursor.try_next().await.ok().expect("Error matching "){
            diagnosis.push(diag);
        }
        Ok(diagnosis)
    }
    pub async fn update(db:&Database, id:String, mut new_diag:&Diagnosis)->Result<UpdateResult, Error>{
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":object_id};
        let collection = db.collection::<Diagnosis>(COLLECTION_NAME);
        let new_doc = doc! {
            "$set":{
                "note":new_diag.note.to_owned(),
                "symptoms":new_diag.symptoms.to_owned(),
                "prescription": new_diag.prescription.to_owned(),
                "updated_at": new_diag.updated_at.to_owned()
            }
        };
        let updated_doc = collection.update_one(filter,new_doc, None )
            .await
            .ok().expect("Error updating diagnosis");

        Ok(updated_doc)
    }
}