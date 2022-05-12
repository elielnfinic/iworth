use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;


#[derive(Deserialize)]
struct Debt{
    name : String,
    value : f32
}

#[get("/debts")]
pub async fn get_debts() -> impl Responder{
    format!("Here are all debts")
}

#[post("/debt/add")]
pub async fn add_debt(debt : web::Json<Debt>) -> impl Responder{
    format!("Adding debt")
}