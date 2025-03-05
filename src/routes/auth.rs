use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;

pub async fn login() -> impl Responder {
    // Crear cliente HTTP
    let client = Client::new();
    let response = client
        .post("http://pm_auth:8080/api/auth/login")
        .send()
        .await
        .unwrap(); 

    let data: Value = response.json().await.unwrap(); // Convertir response a JSON
    HttpResponse::Ok().json(data)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login));
}
