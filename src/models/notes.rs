use bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Note {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub attachments: Vec<Attachment>,
    pub version_history: Vec<Version>,
    pub export_options: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CreateAndUpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub attachments: Option<Vec<Attachment>>,
    pub version_history: Option<Vec<Version>>,
    pub export_options: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Attachment {
    pub id: String,
    pub file_name: String,
    pub file_type: String,
    pub file_size: i64,
    pub url: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Version {
    pub version_id: String,
    pub created_at: String,
    pub content: String,
}
