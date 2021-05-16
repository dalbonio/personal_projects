// main.rs
extern crate juniper;

use std::io;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web ::{middleware, web, App, HttpResponse, Error, HttpRequest, HttpServer, Responder};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql;
use graphql::graphql_schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = std::sync::Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute_sync(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8080/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

