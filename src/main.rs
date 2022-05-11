#[macro_use]
extern crate dotenv_codegen;

use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Serialize;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Serialize)]
struct Liability{
    name : String,
    value : f32
}

#[derive(Serialize)]
struct Debt{
    name : String,
    value : f32
}

#[get("/liabities")]
async fn get_liabilities() -> impl Responder{
    format!("Here are all liabilities")
}

#[post("/liabity/add")]
async fn add_liabily(liability : web::Json<Liability>) -> impl Responder{
    format!("Hi ze world {:?}",liability)
}

#[get("/debts")]
async fn get_debts() -> impl Responder{
    format!("Here are all debts")
}

#[post("/debt/add")]
async fn add_debt(debt : web::Json<Debt>) -> impl Responder{
    format!("Saving debt {:?}",debt)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
   

    HttpServer::new(|| {
        App::new()
            .service(get_liabilities)
            .service(add_liabily)
            .service(get_debts)
            .service(add_debt)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}