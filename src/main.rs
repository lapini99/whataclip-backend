mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json, launch, routes};

use api::user_api::create_user;
use repository::mongodb_repo::MongoRepo;

use crate::api::user_api::get_user;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello, world!")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello, create_user, get_user])
}