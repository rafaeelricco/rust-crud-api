use std::net::TcpListener;
use log::info;
 
mod server;
mod api;
mod config;
mod models;

use server::run;
use config::db;
 
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let address = dotenv::var("address").expect("A variável de ambiente 'address' não está definida. Por favor, defina-a no seu arquivo .env.");
    let db_url = dotenv::var("db_url").expect("A variável de ambiente 'db_url' não está definida. Por favor, defina-a no seu arquivo .env.");

    let listener = TcpListener::bind(address.clone()).expect("Failed to bind to the listener");

    let db_pool = db::init_db_pool(&db_url).await.expect("Erro ao inicializar o pool de conexões do MongoDB.");
    let db = db_pool.database("notes_collection");

    info!("Starting server at http://{}", address);
    run(listener, db)?.await
}
