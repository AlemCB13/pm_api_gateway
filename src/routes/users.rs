use actix_web::{web, HttpResponse, Responder};
use reqwest::Client;
use serde_json::Value;

pub async fn get_user(user_id: web::Path<i32>) -> impl Responder {
    let client = Client::new();
    let response = client
        .get(format!("http://pm_user_management:8081/api/users/{}", user_id))
        .send()
        .await
        .unwrap();

    let data: Value = response.json().await.unwrap();
    HttpResponse::Ok().json(data)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/users/{user_id}", web::get().to(get_user));
}
