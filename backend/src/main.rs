use std::process;
use actix_web::{HttpServer, App};
use db::postgres;
use dotenv::dotenv;

pub mod db;
pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    postgres::connect().await.unwrap_or_else(|e| {
        println!("Postgres Connection Error: \n{}", e);
        process::exit(1)
    });

    println!("Connected to Postgres");

    HttpServer::new(|| {
        App::new()
            .configure(api::config_api)   
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
