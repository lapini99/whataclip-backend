use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: ObjectId,
    pub mail: String,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub current_families: Vec<serde_json::Value>,
}