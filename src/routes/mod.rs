use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use auth::Claims;
use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;

pub mod auth;

pub async fn version() -> impl Responder {
    HttpResponse::Ok().json(json!({"version": env!("CARGO_PKG_VERSION")}))
}

pub async fn proxy(
    req: HttpRequest,
    mut body: web::Payload,
    client: web::Data<Arc<Client>>,
    claims: web::ReqData<Claims>,
) -> Result<HttpResponse, actix_web::Error> {
    // Construct the URL for the upstream server

    println!("claims: {:?}", claims);

    let forward_address =
        std::env::var("GATEWAY_FORWARD_ADDRESS").expect("GATEWAY_FORWARD_ADDRESS not specified");
    let forward_url = format!("{}{}", forward_address, req.uri());

    // Create a new request to the upstream server
    let mut forwarded_req = client.request(
        reqwest::Method::from_bytes(req.method().as_str().as_bytes()).unwrap(),
        &forward_url,
    );

    // Forward the original request headers
    for (header_name, header_value) in req.headers().iter() {
        if header_name != "host" {
            forwarded_req = forwarded_req.header(header_name.as_str(), header_value.as_bytes());
        }
    }

    // Add Connection: keep-alive header
    forwarded_req = forwarded_req.header("Connection", "keep-alive");

    // Collect the body
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    // Send the request and get the response
    let res = forwarded_req
        .body(bytes.freeze())
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Create the response to send back to the client
    let mut client_resp = HttpResponse::build(StatusCode::from_u16(res.status().as_u16()).unwrap());

    // Forward the response headers
    for (header_name, header_value) in res.headers().iter() {
        client_resp.insert_header((header_name.as_str(), header_value.as_bytes()));
    }

    // Stream the response body
    Ok(client_resp.streaming(res.bytes_stream().map(|result| {
        result
            .map(|bytes| Bytes::from(bytes))
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))
    })))
}
