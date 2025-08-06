use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::video_model::Video;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Family {
    #[serde(default)]
    pub id: ObjectId,
    pub name: String,
    pub videos: Vec<Video>,
    pub members: Vec<String>,
}