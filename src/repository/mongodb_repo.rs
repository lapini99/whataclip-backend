use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult},
    sync::{Client, Collection},
};

use crate::models::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Self{
        dotenv().ok();
        let uri = match env::var("DB_URL") {
            Ok(val) => val.to_string(),
            Err(_) => format!("Error loading env variable DB"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("whataclip");
        let col: Collection<User> = db.collection("users");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: ObjectId::new(),
            mail: new_user.mail,
            username: new_user.username,
            password: new_user.password,
            current_families: new_user.current_families,
        };
    
        let user = self.col.insert_one(new_doc, None).ok().expect("Failed to insert user");
        Ok(user)
    }

    pub fn get_user(&self, username: &String) -> Result<User, Error> {
        let filter = doc! { "username": username };
        let user = self.col.find_one(filter, None).ok().expect("Failed to find user");
        Ok(user.unwrap())
    }
}
