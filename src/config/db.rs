#[allow(unused_imports)]
use std::sync::Arc;

use log::info;
use mongodb::{options::ClientOptions, Client};

pub type Pool = Arc<Client>;

pub async fn init_db_pool(url: &str) -> Result<Pool, mongodb::error::Error> {
    info!("Connecting to MongoDB...");

    let mut client_options = ClientOptions::parse(url).await?;
    client_options.app_name = Some("MyApp".to_string());

    let client = Client::with_options(client_options)?;

    Ok(Arc::new(client))
}