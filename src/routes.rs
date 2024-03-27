
use actix_web::{web, Responder, HttpResponse};
use mongodb::Database; 
use futures::stream::StreamExt;
use crate::models::{Note, NewNote};

pub async fn create_note(_db: web::Data<Database>, _info: web::Json<Note>) -> impl Responder {
    HttpResponse::Ok().json("Note created successfully")
}

pub async fn list_notes(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<Note>("Notes");
    let cursor = collection.find(None, None).await.expect("Error trying to get notes");
    let notes: Vec<Result<Note, mongodb::error::Error>> = cursor.collect().await;
    let notes: Vec<Note> = notes.into_iter().filter_map(Result::ok).collect();
    HttpResponse::Ok().json(notes)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/notes")
            .route(web::post().to(create_note))
            .route(web::get().to(list_notes)),
    );
}