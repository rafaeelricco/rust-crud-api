use log::info;
use mongodb::{options::ClientOptions, Client, Database};
use std::sync::Arc;

pub type Pool = Arc<Client>;
pub type SharedDatabase = Arc<Database>;

pub async fn init_db_pool(url: &str) -> Result<Pool, mongodb::error::Error> {
    info!("Connecting to MongoDB...");

    let mut client_options = ClientOptions::parse(url).await?;
    client_options.app_name = Some("actix-web-mongodb".to_string());

    let client = Client::with_options(client_options)?;

    Ok(Arc::new(client))
}

pub async fn get_db() -> SharedDatabase {
    let address = dotenv::var("DB_URL").expect("Environment variable 'DB_URL' is not set. Please define it in your .env file.");

    let mut client_options = ClientOptions::parse(&address)
        .await
        .expect("Failed to initialize MongoDB client options.");
    client_options.app_name = Some("actix-web-mongodb".to_string());

    let client =
        Client::with_options(client_options).expect("Failed to initialize MongoDB client.");
    let db = client.database("rust-actix-web-mongodb");

    Arc::new(db)
}
