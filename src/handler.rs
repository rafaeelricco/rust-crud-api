use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::users::User;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
}

pub async fn authenticate(user: web::Json<User>) -> impl Responder {
    let my_claims = Claims {
        sub: user.username.clone(),
        company: "ACME".to_owned(),
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    HttpResponse::Ok().json(json!({ "token": token }))
}
