use actix_web::{web, HttpResponse};
use reqwest::Client;
use serde_json::Value;

// Obtener un usuario por ID
pub async fn get_user(user_id: web::Path<i32>) -> HttpResponse {
    let client = Client::new();
    let response = client.get(format!("http://pm_user_management:8082/users/{}", user_id))
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Crear un usuario
pub async fn create_user() -> HttpResponse {
    let client = Client::new();
    let response = client.post("http://pm_user_management:8082/users")
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Actualizar un usuario
pub async fn update_user(user_id: web::Path<i32>) -> HttpResponse {
    let client = Client::new();
    let response = client.put(format!("http://pm_user_management:8082/users/{}", user_id))
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Eliminar un usuario
pub async fn delete_user(user_id: web::Path<i32>) -> HttpResponse {
    let client = Client::new();
    let response = client.delete(format!("http://pm_user_management:8082/users/{}", user_id))
        .send()
        .await
        .unwrap();

    let json_response: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(json_response)
}

// Configuración de rutas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/users/{user_id}", web::get().to(get_user))
        .route("/users", web::post().to(create_user))
        .route("/users/{user_id}", web::put().to(update_user))
        .route("/users/{user_id}", web::delete().to(delete_user));
}