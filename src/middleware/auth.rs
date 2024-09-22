use actix_web::{
    dev::ServiceRequest,
    error::ErrorUnauthorized,
    http::header::{HeaderName, HeaderValue},
    Error, HttpMessage,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{DateTime, TimeZone, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::routes::auth::Claims;

pub async fn auth_middleware(
    mut req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    let key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::default(),
    ) {
        Ok(decoded) => {
            req.extensions_mut().insert(decoded.claims.clone());
            let datetime: DateTime<Utc> =
                Utc.timestamp_millis_opt(decoded.claims.exp as i64).unwrap();
            let expiry = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
            req.headers_mut().insert(
                HeaderName::from_static("x-token-expiry"),
                HeaderValue::from_str(&expiry).unwrap(),
            );
            Ok(req)
        }
        Err(_) => Err((ErrorUnauthorized("Invalid token"), req)),
    }
}
