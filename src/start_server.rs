use crate::routes::*;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::Database;
use std::net::TcpListener;
use actix_web::dev::Server;
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
struct ApiInfo {
    api: &'static str,
    version: &'static str,
    database: Option<String>,
    date_created: String
}

pub async fn root() -> impl Responder {

    const API_VERSION: &str = env!("CARGO_PKG_VERSION");
    const API_NAME: &str = env!("CARGO_PKG_NAME");
    
    let date_created: String = Utc::now().format("%d-%m-%Y").to_string();


    let api_infos: ApiInfo = ApiInfo {
        api: API_NAME,
        version: API_VERSION,
        date_created: date_created,
        database: None
    };

    HttpResponse::Ok().json(api_infos)

}


pub fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(root)) // Adicione esta linha para a rota raiz
            .route("/createNote", web::post().to(create_note_route))
            // Adicione as rotas para read_note, update_note e delete_note aqui.
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
