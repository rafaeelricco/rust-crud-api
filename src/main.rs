use dotenv;
use mongodb::{Client, options::ClientOptions};
use rust_crud_api::{models::create_title_unique, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let address = dotenv::var("address").unwrap();
    let db_url = dotenv::var("db_url").unwrap();
    let listener = TcpListener::bind(address.clone()).expect("Failed to bind to the listener");
    let mut client_options = ClientOptions::parse(db_url).await.expect("Failed to connect to the server");
    client_options.app_name = Some("RustCurd".to_string());
    let client = Client::with_options(client_options).expect("Failed to create client");
    let db = client.database("RUSTcurd");
    create_title_unique(&client).await;
    println!("Server is running at {}", address);
    run(listener, db)?.await
}
