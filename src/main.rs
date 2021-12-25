mod config;
mod controllers;
use actix_web::{App, HttpServer};
use crate::config::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind("192.168.1.100:8080").unwrap()
    .run()
    .await
}
