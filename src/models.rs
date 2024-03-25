use serde::{Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;
use mongodb::options::{IndexOptions};
use mongodb::IndexModel;
use mongodb::Client;

use bson::doc;

#[derive(Deserialize, Serialize, Debug)]
pub struct Note {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
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
