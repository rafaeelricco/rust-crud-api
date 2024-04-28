use actix_web::{web, HttpResponse, Responder};
use bson::doc;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::users::User;
use crate::{db::mongodb::get_db, models::users::LogoutRequest};

pub async fn register(body: web::Json<User>) -> impl Responder {
    let db = get_db().await;
    let collection = db.collection::<User>("users");

    let user = User {
        id: Some(Uuid::new_v4().to_string()),
        email: body.email.clone(),
        password: body.password.clone(),
        token: None,
    };
    let usr_cl = user.clone();

    let result = collection.insert_one(user, None).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(usr_cl),
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
                )
                .unwrap();
                info!("Token generated: {:?}", token);

                let token_cl = token.clone();

                let filter = doc! { "email": user.email.clone() };
                let update = doc! { "$set": { "token": token } };
                let result = collection.update_one(filter, update, None).await;

                match result {
                    Ok(_) => {
                        let mut user_cl = user.clone();
                        user_cl.token = Some(token_cl);
                        return HttpResponse::Ok().json(user_cl);
                    }
                    Err(e) => {
                        println!("Erro ao atualizar token do usuário: {:?}", e);
                        return HttpResponse::InternalServerError().finish();
                    }
                }
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
