use actix_web::{web, App, HttpServer};
use routes::config;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)  // Configura las rutas
    })
    .bind("0.0.0.0:8080")?      // Puerto del API Gateway
    .run()
    .await
}