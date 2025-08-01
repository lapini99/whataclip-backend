mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json, launch, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};

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
    
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .allowed_methods(
            vec![rocket::http::Method::Get, rocket::http::Method::Post, rocket::http::Method::Put, rocket::http::Method::Delete]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allowed_headers(rocket_cors::AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .expect("CORS configuration error");

    rocket::build()
        .manage(db)
        .mount("/", routes![hello, create_user, get_user])
        .attach(cors)
}