mod config;
mod controllers;
mod entities;

use actix_web::{App, HttpServer};
use sea_orm::Database;
use crate::config::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn=Database::connect("sqlite:app.db").await.unwrap();
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind("192.168.1.100:8080").unwrap()
    .run()
    .await
}
