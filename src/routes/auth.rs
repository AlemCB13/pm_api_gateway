use actix_web::{web, HttpResponse};
use reqwest::Client;

pub async fn login() -> HttpResponse {
    // Hacer una solicitud al microservicio de autenticación (pm_auth)
    let client = Client::new();
    let response = client.post("http://pm_auth:8080/api/auth/login")
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().json(response.json().await.unwrap())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login));
}