use log::info;
use std::{env, net::TcpListener};

mod controller;
mod db;
mod models;
mod routes;
mod server;

use db::mongodb::init_db_pool;
use server::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "info,actix_web=debug");
    env_logger::init();

    let address = dotenv::var("address").expect("A variável de ambiente 'address' não está definida. Por favor, defina-a no seu arquivo .env.");
    let db_url = dotenv::var("db_url").expect("A variável de ambiente 'db_url' não está definida. Por favor, defina-a no seu arquivo .env.");

    let listener = TcpListener::bind(address.clone()).expect("Failed to bind to the listener");

    let db_pool = init_db_pool(&db_url)
        .await
        .expect("Erro ao inicializar o pool de conexões do MongoDB.");
    let db = db_pool.database("rust-actix-web-mongodb");

    info!("Starting server at http://{}", address);
    run(listener, db)?.await
}
