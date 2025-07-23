extern crate rocket;
use rocket::{get, http::Status, serde::json::Json, launch, routes};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello, world!")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}