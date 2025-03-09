use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde_json::Value;

// Login
pub async fn login() -> HttpResponse {
    let client = Client::new();
    let response = client.post("http://pm_auth:8081/login")
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Registro
pub async fn register() -> HttpResponse {
    let client = Client::new();
    let response = client.post("http://pm_auth:8081/register")
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Logout
pub async fn logout() -> HttpResponse {
    let client = Client::new();
    let response = client.post("http://pm_auth:8081/logout")
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Autenticación de dos factores (2FA)
pub async fn two_factor_auth() -> HttpResponse {
    let client = Client::new();
    let response = client.post("http://pm_auth:8081/2fa")
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Configuración de rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/logout", web::post().to(logout))
        .route("/2fa", web::post().to(two_factor_auth));
}