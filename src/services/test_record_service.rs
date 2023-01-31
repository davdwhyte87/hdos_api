use mongodb::bson::doc;
use mongodb::bson::extjson::de::Error;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use mongodb::results::InsertOneResult;
use crate::models::test_record::TestRecord;
use mongodb::bson::extjson::de::Error::DeserializationError;
use futures::stream::TryStreamExt;
const COLLECTION_NAME:&str = "Test Record";

pub struct TestRecordService{

}

impl TestRecordService {
    pub async fn create(db: &Database, test_record: TestRecord) -> Result<InsertOneResult, Error> {
        // Get a handle to a collection in the database.
        let collection = db.collection::<TestRecord>(COLLECTION_NAME);
        let res_diag =collection.insert_one(test_record, None).await.ok().expect("Error creating Test record");
        Ok(res_diag)
    }

    pub async fn get_by_user_id(db:&Database, id:String)->Result<Vec<TestRecord>, Error>{
        let object_id = ObjectId::parse_str(id);
        let object_id = match object_id {
            Ok(object_id)=>{object_id},
            Err(error)=>{
                return Err(DeserializationError {message:"Error decoding id".to_string()})
            }
        };
        let filter = doc! {"patient_id":object_id};
        let collection = db.collection::<TestRecord>(COLLECTION_NAME);
        let mut cursor = collection.find(filter, None).await.ok().expect("Error getting test record");
        let mut test_record:Vec<TestRecord> = Vec::new();

        while let Some(diag)= cursor.try_next().await.ok().expect("Error matching "){
            test_record.push(diag);
        }
        Ok(test_record)
    }

    pub async fn get_by_id(db:&Database, id:String)->Result<Option<TestRecord>, Error>{
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":object_id};
        let collection = db.collection::<TestRecord>(COLLECTION_NAME);
        let user_detail = collection.find_one(filter, None).await.ok().expect("Error getting test record");
        Ok(user_detail)
    }
    
    
  

    // we cannot update test record
}