use actix_web::{web, HttpResponse, Responder};
use bson::doc;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::users::{User, UserLoginResponse};
use crate::{db::mongodb::get_db, models::users::LogoutRequest};

pub fn generate_token(user: User) -> String {
    let date_now = Utc::now();
    let expiration_date = date_now + Duration::hours(24);

    let token = encode(
        &Header::default(),
        &{
            let mut jwt = HashMap::new();
            jwt.insert("email", Value::String(user.email.clone()));
            jwt.insert("id", Value::String(user.id.clone().unwrap()));
            jwt.insert("date", Value::String(date_now.to_rfc3339()));
            jwt.insert("exp", Value::Number(expiration_date.timestamp().into()));
            jwt
        },
        &EncodingKey::from_secret("JWT_SECRET".as_ref()),
    );

    token.unwrap()
}

pub async fn update_user_token(user: User, token: String) -> HttpResponse {
    let db = get_db().await;
    let collection = db.collection::<User>("users");

    let filter = doc! { "email": user.email.clone() };
    let update = doc! { "$set": { "token": token } };
    let result = collection.update_one(filter, update, None).await;

    match result {
        Ok(_) => {
            info!("Token atualizado com sucesso.");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Erro ao atualizar token: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn register(body: web::Json<User>) -> impl Responder {
    let validate_email = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    if !validate_email.is_match(&body.email) {
        return HttpResponse::Ok().json(doc! { "message": "O email informado é inválido. Por favor, informe um email válido. Ex: example@gmail.com", "status": 400 });
    }

    let db = get_db().await;
    let collection = db.collection::<User>("users");

    let filter = doc! { "email": body.email.clone() };
    let verify_email = collection.find_one(filter, None).await;

    match verify_email {
        Ok(Some(_)) => {
            return HttpResponse::Ok()
                .json(doc! { "message": "Email já cadastrado.", "status": 409 })
        }
        Err(e) => {
            println!("Erro ao verificar email: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
        _ => {}
    }

    let user = User {
        id: Some(Uuid::new_v4().to_string()),
        email: body.email.clone(),
        password: body.password.clone(),
        token: None,
    };

    let token = generate_token(user.clone());

    update_user_token(user.clone(), token.clone()).await;

    let usr_response = json!({
        "id": user.id.clone().unwrap(),
        "email": user.email.clone(),
        "token": token.clone()
    });

    let result = collection.insert_one(user, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(usr_response),
        Err(e) => {
            println!("Erro ao inserir usuário: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login(body: web::Json<User>) -> impl Responder {
    let db = get_db().await;
    let collection = db.collection::<User>("users");

    let filter = doc! { "email": body.email.clone() };
    let result = collection.find_one(filter, None).await;

    match result {
        Ok(Some(user)) => {
            if user.password == body.password {
                let token = generate_token(user.clone());
                let token_cl = token.clone();

                update_user_token(user.clone(), token.clone()).await;

                HttpResponse::Ok().json(UserLoginResponse { token: token_cl })
            } else {
                return HttpResponse::Unauthorized().finish();
            }
        }
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("Erro ao buscar usuário: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

pub async fn logout(body: web::Json<LogoutRequest>) -> impl Responder {
    let db = get_db().await;
    let collection = db.collection::<User>("users");

    let filter = doc! { "email": body.email.clone() };
    let update = doc! { "$set": { "token": null } };
    let result = collection.update_one(filter, update, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json({
            let filter = doc! { "email": body.email.clone() };
            let user = collection.find_one(filter, None).await.unwrap().unwrap();
            user
        }),
        Err(e) => {
            println!("Erro ao deslogar usuário: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
