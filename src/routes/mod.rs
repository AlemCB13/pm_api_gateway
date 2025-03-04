use actix_web::web;

pub mod auth;
pub mod users;
pub mod projects;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::config)
            .configure(users::config)
            .configure(projects::config),
    );
}