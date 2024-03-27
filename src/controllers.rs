use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize};
use mongodb::Database;
use crate::models::{Note};

#[derive(Deserialize)]
pub struct NoteData {
    pub title: String,
    pub content: String,
}

pub async fn create_note(db: web::Data<Database>, info: web::Json<NoteData>) -> impl Responder {
    let data = Note {
        id: None,
        title: info.title.clone(),
        content: info.content.clone(),
    };
    let collection = db.collection::<Note>("Notes");
    match collection.insert_one(data, None).await {
        Ok(_) => HttpResponse::Ok().json("Note created successfully"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {}", err)),
    }
}

// Implementações para read_note, update_note e delete_note seguirão um padrão similar.
