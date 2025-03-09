use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde_json::Value;

// Crear un proyecto
pub async fn create_project() -> HttpResponse {
    let client = Client::new();
    let response = client.post("http://pm_project_management:8083/projects")
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Obtener un proyecto por ID
pub async fn get_project(project_id: web::Path<i32>) -> HttpResponse {
    let client = Client::new();
    let response = client.get(format!("http://pm_project_management:8083/projects/{}", project_id))
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Actualizar un proyecto
pub async fn update_project(project_id: web::Path<i32>) -> HttpResponse {
    let client = Client::new();
    let response = client.put(format!("http://pm_project_management:8083/projects/{}", project_id))
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Eliminar un proyecto
pub async fn delete_project(project_id: web::Path<i32>) -> HttpResponse {
    let client = Client::new();
    let response = client.delete(format!("http://pm_project_management:8083/projects/{}", project_id))
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Configuración de rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/projects", web::post().to(create_project))
        .route("/projects/{project_id}", web::get().to(get_project))
        .route("/projects/{project_id}", web::put().to(update_project))
        .route("/projects/{project_id}", web::delete().to(delete_project));
}