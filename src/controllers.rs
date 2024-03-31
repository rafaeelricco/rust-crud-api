use crate::models::{NewNote, Note};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use futures::stream::StreamExt;
use mongodb::Database;
use mongodb::{bson::doc, Collection};
use serde_json::json;
use uuid::Uuid;

pub async fn create_note(db: web::Data<Database>, info: web::Json<NewNote>) -> impl Responder {
    let collection = db.collection::<Note>("Notes");

    let note = Note {
        id: Uuid::new_v4().to_string(),
        title: info.title.clone(),
        content: info.content.clone(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        tags: info.tags.clone().unwrap_or_default(),
        categories: info.categories.clone().unwrap_or_default(),
        attachments: vec![],
        version_history: vec![],
        export_options: vec![],
    };
    println!("Nota a ser inserida: {:?}", note);

    let note_clone = note.clone();

    let result = collection.insert_one(note, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(note_clone),
        Err(e) => {
            println!("Erro ao inserir nota: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn list_notes(db: web::Data<Database>) -> impl Responder {
    let collection = db.collection::<Note>("Notes");
    let cursor = collection
        .find(None, None)
        .await
        .expect("Error trying to get notes");
    let notes: Vec<Result<Note, mongodb::error::Error>> = cursor.collect().await;
    let notes: Vec<Note> = notes.into_iter().filter_map(Result::ok).collect();

    println!("Notas listadas: {:?}", notes);

    HttpResponse::Ok().json(notes)
}

pub async fn get_note_by_id(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(uuid) => {
            println!("UUID convertido: {}", uuid);
            uuid
        }
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };

    println!("Buscando nota com id: {:?}", uuid);

    let collection: Collection<Note> = db.collection("Notes");

    let uuid_bson = uuid.to_string();
    print!("UUID BSON: {:?}", uuid_bson);

    let filter = doc! { "id": uuid_bson };
    print!("Filtro: {:?}", filter);

    let note = collection.find_one(filter, None).await;
    println!("Nota: {:?}", note);

    match note {
        Ok(Some(note)) => {
            println!("Nota encontrada: {:?}", note);
            HttpResponse::Ok().json(note)
        }
        Ok(None) => {
            println!("Nota não encontrada.");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            println!("Erro ao buscar nota: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_note(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(uuid) => {
            println!("UUID convertido: {}", uuid);
            uuid
        }
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };

    println!("Buscando nota com id: {:?}", uuid);

    let collection: Collection<Note> = db.collection("Notes");

    let uuid_bson = uuid.to_string();
    print!("UUID BSON: {:?}", uuid_bson);

    let filter = doc! { "id": uuid_bson };
    print!("Filtro: {:?}", filter);

    let note = collection.find_one_and_delete(filter, None).await;
    println!("Nota: {:?}", note);

    match note {
        Ok(Some(note)) => {
            println!("Nota deletada: {:?}", note);
            let res = json!({
                "message": "Nota deletada com sucesso!",
                "nota": note
            });
            HttpResponse::Ok().json(res)
        }
        Ok(None) => {
            println!("Nota não encontrada.");
            let res = json! ({
                "message": "Oopss! Nota não encontrada. Verifique o ID informado."
            });
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            println!("Erro ao deletar nota: {:?}", e);
            let res = json! ({
                "message": "Oopss! Ocorreu um erro ao deletar a nota. Verifique o ID informado."
            });
            HttpResponse::Ok().json(res)
        }
    }
}

pub async fn delete_all_notes(db: web::Data<Database>) -> impl Responder {
    let collection: Collection<Note> = db.collection("Notes");
    let action = collection.delete_many(doc! {}, None).await;

    match action {
        Ok(delete_result) => {
            let deleted_count = delete_result.deleted_count;
            let res = json!({
                "message": format!("{} notas foram excluídas com sucesso.", deleted_count)
            });
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            println!("Erro ao excluir todas as notas: {:?}", e);
            let res =
                json!({ "message": "Oopss! Ocorreu um erro ao tentar excluir todas as notas." });
            HttpResponse::InternalServerError().json(res)
        }
    }
}
