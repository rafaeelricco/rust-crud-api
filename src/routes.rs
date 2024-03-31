use crate::controllers::{
    delete_all_notes, delete_note_by_id, get_all_notes, get_note_by_id, patch_note, post_new_note,
};
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct ApiInfo {
    api: &'static str,
    version: &'static str,
    database: Option<String>,
    date_created: String,
}

pub async fn root() -> impl Responder {
    const API_VERSION: &str = env!("CARGO_PKG_VERSION");
    const API_NAME: &str = env!("CARGO_PKG_NAME");

    let date_created: String = Utc::now().format("%d-%m-%Y").to_string();

    let api_infos: ApiInfo = ApiInfo {
        api: API_NAME,
        version: API_VERSION,
        date_created: date_created,
        database: None,
    };

    HttpResponse::Ok().json(api_infos)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(root)));
    cfg.service(web::resource("/health").route(web::get().to(root)));
    cfg.service(
        web::scope("/notes")
            .route("", web::post().to(post_new_note))
            .route("", web::get().to(get_all_notes))
            .route("/{id}", web::get().to(get_note_by_id))
            .route("/{id}", web::delete().to(delete_note_by_id))
            .route("/{id}", web::patch().to(patch_note)),
    );
    cfg.service(web::resource("/reset_notes").route(web::get().to(delete_all_notes)));
}
