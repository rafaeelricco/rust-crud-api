use actix_web::{web, App, HttpServer};
use mongodb::{Database};
use std::net::TcpListener;
use actix_web::dev::Server;
use crate::routes::*;

pub fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .route("/createNote", web::post().to(create_note_route))
            // Adicione as rotas para read_note, update_note e delete_note aqui.
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
