#[macro_use]
extern crate dotenv_codegen;

use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use dotenv::dotenv;



#[get("/liabities")]
async fn get_liabilities() -> impl Responder{
    
}

#[post("/liabity/add")]
async fn add_liabily() -> impl Responder{

}

#[get("/debts")]
async fn get_debts() -> impl Responder{

}

#[post("/debt/add")]
async fn add_debt -> impl Responder{

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