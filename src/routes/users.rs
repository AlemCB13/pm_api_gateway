use actix_web::{web, HttpResponse};
use reqwest::Client;

pub async fn get_user(user_id: web::Path<i32>) -> HttpResponse {
    // Hacer una solicitud al microservicio de gestión de usuarios (pm_user_management)
    let client = Client::new();
    let response = client.get(format!("http://pm_user_management:8081/api/users/{}", user_id))
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().json(response.json().await.unwrap())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/users/{user_id}", web::get().to(get_user));
}