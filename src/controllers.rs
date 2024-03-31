use crate::models::{CreateAndUpdateNote, Note};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use futures::stream::StreamExt;
use mongodb::options::FindOptions;
use mongodb::Database;
use mongodb::{bson::doc, Collection};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

fn convert_uuid(uuid: String) -> Uuid {
    match Uuid::parse_str(&uuid) {
        Ok(uuid) => {
            println!("UUID convertido: {}", uuid);
            uuid
        }
        Err(_) => panic!("Invalid UUID"),
    }
}

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<i64>,
    page_size: Option<i64>,
    tag: Option<String>,
    category: Option<String>,
    created_after: Option<String>,
}

pub async fn get_all_notes(
    db: web::Data<Database>,
    params: web::Query<PaginationParams>,
) -> impl Responder {
    let collection: Collection<Note> = db.collection("Notes");

    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let skip = (page - 1) * page_size;

    let options = FindOptions::builder()
        .skip(Some(skip as u64))
        .limit(Some(page_size as i64))
        .build();

    let mut filter = doc! {};
    if let Some(tag) = &params.tag {
        filter.insert("tags", tag);
    }
    if let Some(category) = &params.category {
        filter.insert("categories", category);
    }
    if let Some(created_after) = &params.created_after {
        filter.insert("created_at", doc! { "$gte": created_after });
    }

    let cursor = collection
        .find(filter, Some(options))
        .await
        .expect("Error trying to get notes");

    let notes: Vec<Result<Note, mongodb::error::Error>> = cursor.collect().await;
    let notes: Vec<Note> = notes.into_iter().filter_map(Result::ok).collect();

    println!("Notas listadas: {:?}", notes);

    HttpResponse::Ok().json(notes)
}

pub async fn get_note_by_id(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let collection: Collection<Note> = db.collection("Notes");

    let uuid_bson = convert_uuid(id.into_inner()).to_string();
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

pub async fn post_new_note(
    db: web::Data<Database>,
    info: web::Json<CreateAndUpdateNote>,
) -> impl Responder {
    let collection = db.collection::<Note>("Notes");

    let note = Note {
        id: Uuid::new_v4().to_string(),
        title: info.title.clone(),
        content: info.content.clone(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        tags: info.tags.clone(),
        categories: info.categories.clone(),
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

pub async fn patch_note_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    info: web::Json<CreateAndUpdateNote>,
) -> impl Responder {
    let collection: Collection<Note> = db.collection("Notes");
    let uuid_bson = convert_uuid(id.into_inner()).to_string();
    let filter = doc! { "id": uuid_bson };
    let note = collection.find_one(filter, None).await;

    match note {
        Ok(Some(mut note)) => {
            println!("Nota encontrada: {:?}", note);

            if let Some(title) = info.title.clone() {
                note.title = Some(title);
            }

            if let Some(content) = info.content.clone() {
                note.content = Some(content);
            }

            if let Some(tags) = info.tags.clone() {
                note.tags = Some(tags);
            }

            if let Some(categories) = info.categories.clone() {
                note.categories = Some(categories);
            }

            note.updated_at = Utc::now().to_rfc3339();

            let id_cl = convert_uuid(note.id.clone()).to_string();
            println!("ID para atualização: {:?}", id_cl);

            let note_updated = note.clone();

            let result = collection
                .replace_one(doc! { "id": id_cl }, note, None)
                .await;
            println!("Resultado da atualização: {:?}", result);

            let res = json!({
                "message": "Nota atualizada com sucesso!",
                "nota": note_updated
            });

            match result {
                Ok(_) => HttpResponse::Ok().json(res),
                Err(e) => {
                    println!("Erro ao atualizar nota: {:?}", e);
                    let res = json!({
                        "message": "Oopss! Ocorreu um erro ao atualizar a nota.",
                        "error": e.to_string()
                    });
                    HttpResponse::Ok().json(res)
                }
            }
        }
        Ok(None) => {
            println!("Nota não encontrada.");
            let res = json!({
                "message": "Oopss! Nota não encontrada. Verifique o ID informado."
            });
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            println!("Erro ao buscar a nota: {:?}", e);
            let res = json!({
                "message": "Oopss! Ocorreu um erro ao buscar a nota. Verifique o ID informado.",
                "error": e.to_string()
            });
            HttpResponse::Ok().json(res)
        }
    }
}

pub async fn delete_note_by_id(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
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
