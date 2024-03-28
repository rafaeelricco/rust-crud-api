use crate::controllers::{create_note, list_notes};
use actix_web::{web, HttpResponse, Responder};
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
    cfg.service(
        web::resource("/notes")
            .route(web::post().to(create_note))
            .route(web::get().to(list_notes)),
    );
}
