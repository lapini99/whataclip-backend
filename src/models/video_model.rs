use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: mongodb::bson::oid::ObjectId,
    pub title: String,
    pub description: String,
    pub uploader: String,
    pub upload_date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub url: String,
}
