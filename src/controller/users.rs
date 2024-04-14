#![allow(unused_variables)]
use actix_web::{web, Responder};
use mongodb::Database;

pub async fn login(db: web::Data<Database>) -> impl Responder {
    return "Login";
}
pub async fn register(db: web::Data<Database>) -> impl Responder {
    return "Register";
}
pub async fn logout(db: web::Data<Database>) -> impl Responder {
    return "Logout";
}
