use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health_check);
}