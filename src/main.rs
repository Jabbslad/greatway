use std::{sync::Arc, time::Duration};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;

mod middleware;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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
            .wrap(middleware::headers::Headers)
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(routes::auth::register))
                    .route("/login", web::post().to(routes::auth::login)),
            )
            .service(web::resource("/version").route(web::get().to(routes::version)))
            .service(
                web::scope("")
                    .wrap(middleware::auth::Auth)
                    .default_service(web::route().to(routes::proxy)),
            )
    })
    .bind((host.clone(), port))?;
    println!("Server starting at: {}:{}", host, port);
    server.run().await
}
