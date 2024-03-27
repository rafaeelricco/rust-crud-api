use actix_web::{web, Responder, HttpResponse};
use mongodb::Database;
use crate::models::{Note, NewNote};
use futures::stream::StreamExt;

pub async fn create_note(db: web::Data<Database>, info: web::Json<NewNote>) -> impl Responder {
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


pub async fn list_notes(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<Note>("Notes");
    let cursor = collection.find(None, None).await.expect("Error trying to get notes");
    let notes: Vec<Result<Note, mongodb::error::Error>> = cursor.collect().await;
    let notes: Vec<Note> = notes.into_iter().filter_map(Result::ok).collect();
    HttpResponse::Ok().json(notes)
}