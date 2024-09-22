use std::{sync::Arc, time::Duration};

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use guards::role_guard::RoleGuard;
use middleware::auth;
use models::user::Role;
use reqwest::Client;
use routes::auth::{login, register};

mod guards {
    pub mod role_guard;
}
mod middleware;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let client = Arc::new(
        Client::builder()
            .user_agent("greatway/1.0")
            .timeout(Duration::from_secs(30))
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(100)
            .build()
            .expect("Failed to create reqwest Client"),
    );

    let host = std::env::var("GREATWAY_LISTEN_ADDRESS").unwrap_or("0.0.0.0".into());
    let port: u16 = std::env::var("GREATWAY_LISTEN_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .configure(config)
    })
    .bind((host.clone(), port))?;
    server.run().await
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(web::resource("/register").route(web::post().to(register)))
            .service(web::resource("/login").route(web::post().to(login))),
    )
    .service(web::resource("/version").route(web::get().to(routes::version)))
    .service(
        web::scope("")
            .wrap(HttpAuthentication::bearer(auth::auth_middleware))
            .service(
                web::scope("")
                    .guard(RoleGuard(vec![Role::Admin]))
                    .default_service(web::route().to(routes::proxy)),
            ),
    );
}
