use actix_web::{App, HttpServer};

mod app;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(app::init)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
