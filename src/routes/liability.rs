use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Liability{
    name : String,
    value : f32
}


#[get("/liabities")]
pub async fn get_liabilities() -> impl Responder{
    format!("Here are all liabilities")
}

#[post("/liabity/add")]
pub async fn add_liabily(liability : web::Json<Liability>) -> impl Responder{
    format!("Adding liability")
}


