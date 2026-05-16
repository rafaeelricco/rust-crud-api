use db::mongodb::init_db_pool;
use log::info;
use server::run;
use std::{env, net::TcpListener};

mod controller;
mod db;
mod middleware;
mod models;
mod routes;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "info,actix_web=debug");
    env_logger::init();
    info!("Starting server...");

    let address = dotenv::var("HOST").expect("Environment variable 'HOST' is not set. Please define it in your .env file.");
    let db_url = dotenv::var("DB_URL").expect("Environment variable 'DB_URL' is not set. Please define it in your .env file.");

    let listener = TcpListener::bind(address.clone()).expect("Failed to bind to the listener");

    let db_pool = init_db_pool(&db_url)
        .await
        .expect("Failed to initialize the MongoDB connection pool.");
    let db = db_pool.database("rust-actix-web-mongodb");

    info!("Starting server at http://{}", address);
    run(listener, db)?.await
}
