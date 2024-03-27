use actix_web::web;
use crate::controllers::{create_note, list_notes}; 

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/notes")
            .route(web::post().to(create_note)) 
            .route(web::get().to(list_notes)), 
    );
}
