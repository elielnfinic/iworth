

use mongodb::{bson::doc, options::ClientOptions, Client};

use dotenv::dotenv;
use std::env;

#[tokio::main]
pub async fn connect() -> mongodb::error::Result<()> {
    let mut client_options =
        ClientOptions::parse(dotenv!("MONGO_URI"))
            .await?;
    client_options.app_name = Some("Rust Demo".to_string());

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}