#[macro_use]
extern crate dotenv_codegen;

use actix_web::{get, web, App, HttpServer, Responder};
use std::env;
use dotenv::dotenv;



#[get("/liabities")]
async fn get_liabilities() -> impl Responder{
    
}

#[post("/liabitie")]

#[post("/getdebt")]

#[post("/debt")]

#[actix_web::main]
async fn main() -> std::io::Result<()>{
   

    HttpServer::new(|| {
        App::new()
            .service(get_liabilities)
    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}