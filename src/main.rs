use actix_web::{App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::projects::create_project)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}