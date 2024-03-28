use crate::routes::{config, root};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use mongodb::Database;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .configure(config)
            .app_data(db.clone())
            .route("/", web::get().to(root))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
