use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;

pub async fn create_project() -> impl Responder {
    let client = Client::new();
    let response = client
        .post("http://pm_project_management:8082/api/projects")
        .send()
        .await
        .unwrap();

    let data: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(data)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/projects", web::post().to(create_project));
}
