use serde::{Deserialize, Serialize};
use bson::doc;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Note {
    pub id: Uuid,
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
