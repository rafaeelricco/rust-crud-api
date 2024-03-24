use serde::{Deserialize, Serialize};
use mongodb::options::{IndexOptions};
use mongodb::IndexModel;
use mongodb::Client;
use bson::doc;

#[derive(Deserialize, Serialize)]
pub struct Note {
    pub title: String,
    pub content: String,
}

pub async fn create_title_unique(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! {"title": 1})
        .options(options)
        .build();
    client
        .database("RUSTcurd")
        .collection::<Note>("Notes")
        .create_index(model, None)
        .await
        .expect("error creating index!");
}
