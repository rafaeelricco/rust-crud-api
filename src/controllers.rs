use actix_web::{web, Responder, HttpResponse};
use mongodb::Database;
use uuid::Uuid;
use crate::models::{Note, NewNote};
use futures::stream::StreamExt;

pub async fn create_note(db: web::Data<Database>, info: web::Json<NewNote>) -> impl Responder {

    let collection = db.collection::<Note>("Notes");
    
    let note = Note {
        id: Uuid::new_v4(), 
        title: info.title.clone(),
        content: info.content.clone(),
    };

    let note_clone = note.clone();

    let result = collection.insert_one(note, None).await;

    match result {
        Ok(_) => {
            println!("Nota inserida com sucesso."); 
            HttpResponse::Ok().json(note_clone)
        },
        Err(e) => {
            println!("Erro ao inserir nota: {:?}", e); 
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn list_notes(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<Note>("Notes");
    let cursor = collection.find(None, None).await.expect("Error trying to get notes");
    let notes: Vec<Result<Note, mongodb::error::Error>> = cursor.collect().await;
    let notes: Vec<Note> = notes.into_iter().filter_map(Result::ok).collect();

    println!("Notas listadas: {:?}", notes); 

    HttpResponse::Ok().json(notes)
}
