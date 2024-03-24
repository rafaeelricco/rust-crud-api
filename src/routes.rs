
use actix_web::{web, Responder, HttpResponse};
use mongodb::Database; // Importa o tipo Database do pacote mongodb
use crate::models::Note; // Corrigido para Note em vez de NoteData

// Supondo que create_note está definido neste arquivo
pub async fn create_note(_db: web::Data<Database>, _info: web::Json<Note>) -> impl Responder {
    // Implementação de create_note aqui
    HttpResponse::Ok().json("Note created successfully")
}


pub async fn create_note_route(db: web::Data<Database>, info: web::Json<Note>) -> impl Responder {
    create_note(db, info).await
}
