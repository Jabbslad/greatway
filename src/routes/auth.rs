use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    config::AppConfig,
    db,
    models::user::{Role, User},
};

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

pub async fn register(
    user: web::Json<LoginRequest>,
    data: web::Data<Arc<AppConfig>>,
) -> impl Responder {
    let user = db::create_user(&data.pool, &user.username, &user.password).unwrap();
    db::add_role_to_user(&data.pool, &user.id, Role::User).unwrap();

    HttpResponse::Ok().json(user)
}

pub async fn login(
    user: web::Json<LoginRequest>,
    data: web::Data<Arc<AppConfig>>,
) -> impl Responder {
    let db_user = db::get_user_by_username(&data.pool, &user.username)
        .unwrap()
        .unwrap();

    let roles = db::get_user_roles(&data.pool, &db_user.id).unwrap();

    if verify(&user.password, &db_user.password).unwrap() {
        let token = generate_token(&db_user, &roles);
        HttpResponse::Ok().json(LoginResponse { token })
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub fn generate_token(user: &User, roles: &Vec<Role>) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
        + 3_600_000; // 1 hour in ms

    let claims = Claims {
        sub: user.username.clone(),
        exp: expiration,
        roles: roles.clone(),
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
