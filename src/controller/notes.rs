use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use futures::stream::StreamExt;
use mongodb::options::FindOptions;
use mongodb::Database;
use mongodb::{bson::doc, Collection};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::models::notes::{CreateAndUpdateNote, Note};

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

    println!("Notes listed: {:?}", notes);

    HttpResponse::Ok().json(notes)
}

pub async fn get_note_by_id(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let collection: Collection<Note> = db.collection("Notes");

    let uuid_bson = convert_uuid(id.into_inner()).to_string();
    print!("UUID BSON: {:?}", uuid_bson);

    let filter = doc! { "id": uuid_bson };
    print!("Filter: {:?}", filter);

    let note = collection.find_one(filter, None).await;
    println!("Note: {:?}", note);

    match note {
        Ok(Some(note)) => {
            println!("Note found: {:?}", note);
            HttpResponse::Ok().json(note)
        }
        Ok(None) => {
            println!("Note not found.");
            HttpResponse::NotFound().finish()
        }
        Err(e) => {
            println!("Failed to fetch note: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn post_new_note(
    db: web::Data<Database>,
    info: web::Json<CreateAndUpdateNote>,
) -> impl Responder {
    let collection = db.collection::<Note>("notes");

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
    println!("Note to be inserted: {:?}", note);

    let note_clone = note.clone();
    let result = collection.insert_one(note, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(note_clone),
        Err(e) => {
            println!("Failed to insert note: {:?}", e);
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
            println!("Note found: {:?}", note);

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
            println!("ID for update: {:?}", id_cl);

            let note_updated = note.clone();

            let result = collection
                .replace_one(doc! { "id": id_cl }, note, None)
                .await;
            println!("Update result: {:?}", result);

            let res = json!({
                "message": "Note updated successfully!",
                "note": note_updated
            });

            match result {
                Ok(_) => HttpResponse::Ok().json(res),
                Err(e) => {
                    println!("Failed to update note: {:?}", e);
                    let res = json!({
                        "message": "Oops! An error occurred while updating the note.",
                        "error": e.to_string()
                    });
                    HttpResponse::Ok().json(res)
                }
            }
        }
        Ok(None) => {
            println!("Note not found.");
            let res = json!({
                "message": "Oops! Note not found. Please check the ID provided."
            });
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            println!("Failed to fetch the note: {:?}", e);
            let res = json!({
                "message": "Oops! An error occurred while fetching the note. Please check the ID provided.",
                "error": e.to_string()
            });
            HttpResponse::Ok().json(res)
        }
    }
}

pub async fn delete_note_by_id(db: web::Data<Database>, id: web::Path<String>) -> impl Responder {
    let uuid = match Uuid::parse_str(&id.into_inner()) {
        Ok(uuid) => {
            println!("UUID converted: {}", uuid);
            uuid
        }
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID"),
    };

    println!("Fetching note with id: {:?}", uuid);

    let collection: Collection<Note> = db.collection("Notes");

    let uuid_bson = uuid.to_string();
    print!("UUID BSON: {:?}", uuid_bson);

    let filter = doc! { "id": uuid_bson };
    print!("Filter: {:?}", filter);

    let note = collection.find_one_and_delete(filter, None).await;
    println!("Note: {:?}", note);

    match note {
        Ok(Some(note)) => {
            println!("Note deleted: {:?}", note);
            let res = json!({
                "message": "Note deleted successfully!",
                "note": note
            });
            HttpResponse::Ok().json(res)
        }
        Ok(None) => {
            println!("Note not found.");
            let res = json! ({
                "message": "Oops! Note not found. Please check the ID provided."
            });
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            println!("Failed to delete note: {:?}", e);
            let res = json! ({
                "message": "Oops! An error occurred while deleting the note. Please check the ID provided."
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
                "message": format!("{} notes were deleted successfully.", deleted_count)
            });
            HttpResponse::Ok().json(res)
        }
        Err(e) => {
            println!("Failed to delete all notes: {:?}", e);
            let res =
                json!({ "message": "Oops! An error occurred while trying to delete all notes." });
            HttpResponse::InternalServerError().json(res)
        }
    }
}

fn convert_uuid(uuid: String) -> Uuid {
    match Uuid::parse_str(&uuid) {
        Ok(uuid) => {
            println!("UUID converted: {}", uuid);
            uuid
        }
        Err(_) => panic!("Invalid UUID"),
    }
}
