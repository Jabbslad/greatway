use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

pub async fn register(user: web::Json<LoginRequest>) -> impl Responder {
    let hashed_password = hash(&user.password, 10).unwrap();
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        username: user.username.clone(),
        password: hashed_password,
    };

    HttpResponse::Ok().json(new_user)
}

pub async fn login(user: web::Json<LoginRequest>) -> impl Responder {
    // In a real app, you'd fetch the user from your database
    let stored_user = User {
        id: Uuid::new_v4(),
        username: user.username.clone(),
        password: hash(&user.password, 10).unwrap(), // This is just for demo
    };

    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 hour

    let claims = Claims {
        sub: stored_user.username.clone(),
        exp: expiration,
    };

    if verify(&user.password, &stored_user.password).unwrap() {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();

        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
