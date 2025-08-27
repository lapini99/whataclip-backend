use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use bcrypt::{verify};
use rocket::{post, State, serde::json::Json, http::Status};

use crate::models::jwt_model::{LoginRequest, LoginResponse, Claims};
use crate::repository::mongodb_repo::MongoRepo;

const JWT_SECRET: &str = "your-secret-key-change-this-in-production";

#[post("/login", data = "<login_request>")]
pub fn login(db: &State<MongoRepo>, login_request: Json<LoginRequest>) -> Result<Json<LoginResponse>, Status> {
    let user_result = db.get_user_by_email(&login_request.mail);
    
    match user_result {
        Ok(user) => {
            // For development, using plain text comparison
            // In production, use: verify(&login_request.password, &user.password).unwrap_or(false)
            if login_request.password == user.password {
                let token = generate_jwt(&user.id.to_string())?;
                Ok(Json(LoginResponse {
                    token,
                    user_id: user.id.to_string(),
                }))
            } else {
                Err(Status::Unauthorized)
            }
        },
        Err(_) => Err(Status::Unauthorized)
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

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref()))
        .map_err(|_| Status::InternalServerError)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}