use actix_web::{web, HttpResponse};

pub async fn login() -> HttpResponse {
    HttpResponse::Ok().json("Login endpoint")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/auth/login", web::post().to(login));
}