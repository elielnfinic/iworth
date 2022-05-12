#[macro_use]
extern crate dotenv_codegen;
mod routes;

use actix_web::{get, post, web, App, HttpServer, Responder};
use std::env;
use dotenv::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            .service(routes::liability::get_liabilities)
            .service(routes::liability::add_liabily)
            .service(routes::debt::get_debts)
            .service(routes::debt::add_debt)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}