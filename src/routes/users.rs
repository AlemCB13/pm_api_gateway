use actix_web::{web, HttpResponse};

pub async fn get_user(user_id: web::Path<i32>) -> HttpResponse {
    HttpResponse::Ok().json(format!("User ID: {}", user_id))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/users/{user_id}", web::get().to(get_user));
}