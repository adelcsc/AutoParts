mod config;
mod controllers;
mod entities;
#[cfg(test)]
mod tests;

use std::io::Bytes;
use actix_web::{App, guard, HttpResponse, HttpServer, web};
use actix_web::web::Data;
use async_graphql::{EmptyMutation, EmptySubscription, Schema, SchemaBuilder};
use async_graphql::dataloader::DataLoader;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use crate::config::config;
use crate::entities::{Query, SqliteLoader};

struct AppState {
    schema : Schema<Query,EmptyMutation,EmptySubscription>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn=Database::connect("sqlite:app.db").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .configure(config)
            .service(web::resource("/").guard(guard::Post()).to(graph))
            .service(web::resource("/").guard(guard::Get()).to(playground))
            .app_data(Data::new(AppState{
                schema : Schema::build(Query,EmptyMutation,EmptySubscription)
                    .data(DataLoader::new(SqliteLoader{pool: conn.to_owned()},actix_rt::spawn)).finish(),
            }))
    })
    .bind("192.168.1.100:8080").unwrap()
    .run()
    .await
}

async fn graph(req : GraphQLRequest,data : web::Data<AppState>) ->GraphQLResponse {
    let res = data.schema.execute(req.into_inner()).await;
    res.into()
}
async fn playground() ->HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/")))
}
