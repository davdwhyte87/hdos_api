use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::string::ToString;
use handlebars::Handlebars;
use mongodb::{Client, Database, options::ClientOptions};
use mongodb::bson::extjson::de::Error;
use mongodb::results::InsertOneResult;
use serde_json::{json, Value};
use serde_json::Value::String;

use crate::database::db::db::DB;
use crate::models::helper::EmailData;
use crate::models::user::User;
use crate::utils::send_email::{ACTIVATE_EMAIL, get_body, send_email};

const COLLECTION_NAME:&str = "User";

pub struct UserService{
    client: Client

}

impl UserService{
    pub async fn create_user(db:&Database, user:&User)->Result<InsertOneResult, Error>{
        // Get a handle to a collection in the database.
        let collection = db.collection::<User>(COLLECTION_NAME);

        // let new_user = User{
        //     id:None,
        //     name:user.name
        // };

        //send email
        // let mut file = File::open("html/activate.html").expect("File not found");
        // let mut html_data = String::new();
        // file.read_to_string(&mut html_data);
        let code:u32= 9384;


        let name = user.name.as_str().to_string();

        let mut reg = Handlebars::new();
        let order_email_content = reg.render_template (
            include_str!("../utils/html/activate_new_account.hbs"),
            &serde_json::json!({"name" :name, "code":code})).unwrap();

        let email_data = EmailData{
            subject:"Confirmation code".to_string(),
            to: (*user.email).parse().unwrap(),
            body: order_email_content
        };
        send_email(email_data);
        // Insert data into db.
        let res_user =collection.insert_one(user, None).await.ok().expect("Error creating user");
        Ok(res_user)
    }


}