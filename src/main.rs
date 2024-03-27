use dotenv;
use mongodb::{Client, options::ClientOptions};
use rust_crud_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    let address = dotenv::var("address").expect("A variável de ambiente 'address' não está definida. Por favor, defina-a no seu arquivo .env.");
    let db_url = dotenv::var("db_url").expect("A variável de ambiente 'db_url' não está definida. Por favor, defina-a no seu arquivo .env.");

    let listener = TcpListener::bind(address.clone()).expect("Failed to bind to the listener");
    let mut client_options = ClientOptions::parse(db_url).await.expect("Failed to connect to the server");
    client_options.app_name = Some("Notes".to_string());
    
    let client = Client::with_options(client_options).expect("Failed to create client");
    println!("Connected to the database: {}", dotenv::var("db_url").expect("A variável de ambiente 'db_url' não está definida. Por favor, defina-a no seu arquivo .env."));
    
    let db = client.database("notes_collection");

    println!("Server is running at {}", address);
    run(listener, db)?.await
}
