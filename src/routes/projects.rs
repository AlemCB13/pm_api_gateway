use actix_web::{web, HttpResponse};
use reqwest::Client;

pub async fn create_project() -> HttpResponse {
    // Hacer una solicitud al microservicio de gestión de proyectos (pm_project_management)
    let client = Client::new();
    let response = client.post("http://pm_project_management:8082/api/projects")
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().json(response.json().await.unwrap())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/projects", web::post().to(create_project));
}