use actix_web::{web, HttpResponse};
use serde_json::json;

pub async fn create_project() -> HttpResponse {
    HttpResponse::Ok().json(json!({ "message": "Project created" }))
}