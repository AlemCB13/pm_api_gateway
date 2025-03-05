use actix_web::{App, HttpServer, web};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config) // Se usa la configuración de rutas
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
