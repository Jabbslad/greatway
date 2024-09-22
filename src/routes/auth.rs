use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::{Role, User};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub roles: Vec<Role>,
}

pub async fn register(user: web::Json<LoginRequest>) -> impl Responder {
    let hashed_password = hash(&user.password, 10).unwrap();
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        username: user.username.clone(),
        email: "test@test.com".to_string(),
        password: hashed_password,
        roles: vec![Role::User],
    };

    HttpResponse::Ok().json(new_user)
}

pub async fn login(user: web::Json<LoginRequest>) -> impl Responder {
    // In a real app, you'd fetch the user from your database
    let stored_user = User {
        id: Uuid::new_v4(),
        username: user.username.clone(),
        email: "test@test.com".to_string(),
        password: hash(&user.password, 10).unwrap(), // This is just for demo
        roles: vec![Role::User],
    };

    if verify(&user.password, &stored_user.password).unwrap() {
        let token = generate_token(&stored_user);
        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub fn generate_token(user: &User) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // 1 hour

    let claims = Claims {
        sub: user.username.clone(),
        exp: expiration,
        roles: user.roles.clone(),
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
