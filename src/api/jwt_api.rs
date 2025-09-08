use std::env;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use bcrypt::{verify};
use rocket::{post, State, serde::json::Json, http::Status};

use crate::models::jwt_model::{LoginRequest, LoginResponse, Claims};
use crate::repository::mongodb_repo::MongoRepo;

#[post("/login", data = "<login_request>")]
pub fn login(db: &State<MongoRepo>, login_request: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    println!("Login attempt for email: {}", login_request.mail);
    
    let user_result = db.inner().get_user_by_email(&login_request.mail);
    
    match user_result {
        Ok(user) => {
            println!("User found: {}", user.username);
            println!("Stored password hash: {}", user.password);
            println!("Login password: {}", login_request.password);
            
            // Check if password is already hashed or plain text
            let password_matches = if user.password.starts_with("$2") {
                // Password is bcrypt hashed
                println!("Verifying with bcrypt");
                verify(&login_request.password, &user.password).unwrap_or(false)
            } else {
                // Password is plain text (for development)
                println!("Comparing plain text");
                login_request.password == user.password
            };
            
            if password_matches {
                println!("Password verification successful");
                let token = generate_jwt(&user.id.to_string())?;
                Ok(Json(LoginResponse {
                    token,
                    user_id: user.id.to_string(),
                }))
            } else {
                println!("Password verification failed");
                Err(Status::Unauthorized)
            }
        },
        Err(e) => {
            println!("User not found: {:?}", e);
            Err(Status::Unauthorized)
        }
    }
}

fn generate_jwt(user_id: &str) -> Result<String, Status> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()))
        .map_err(|_| Status::InternalServerError)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}