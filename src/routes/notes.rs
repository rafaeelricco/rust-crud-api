use crate::controller::notes::*;
use actix_web::web;

pub fn configure_note_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notes")
            .route("", web::post().to(post_new_note))
            .route("", web::get().to(get_all_notes))
            .route("/{id}", web::get().to(get_note_by_id))
            .route("/{id}", web::delete().to(delete_note_by_id))
            .route("/{id}", web::patch().to(patch_note_by_id)),
    );
    cfg.service(web::resource("/reset_notes").route(web::get().to(delete_all_notes)));
}
