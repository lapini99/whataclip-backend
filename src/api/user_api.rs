use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, get, post};

#[post("/user", format = "json", data = "<new_user>")]
pub fn create_user(new_user: Json<User>, db: &State<MongoRepo>) -> Result<Json<InsertOneResult>, Status> {
    let user = User {
        id: mongodb::bson::oid::ObjectId::new(),
        mail: new_user.mail.to_owned(),
        username: new_user.username.to_owned(),
        role: new_user.role.to_owned(),
        password: new_user.password.to_owned(),
        avatar: new_user.avatar.to_owned(),
        biography: new_user.biography.to_owned(),
        current_families: new_user.current_families.to_owned(),
    };

    match db.inner().create_user(user) {
        Ok(result) => Ok(Json(result)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user/<username>")]
pub fn get_user(username: String, db: &State<MongoRepo>) -> Result<Json<User>, Status> {
    match db.inner().get_user(&username) {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::NotFound),
    }
}