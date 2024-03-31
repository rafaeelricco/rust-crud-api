use bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Note {
    pub id: String,
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
