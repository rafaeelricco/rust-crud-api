use crate::models::users::User;
use crate::{db::mongodb::get_db, models::users::LogoutRequest};
use actix_web::{web, HttpResponse, Responder};
use bson::doc;
use jsonwebtoken::{encode, EncodingKey, Header};
use log::info;
use uuid::Uuid;

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
                let date_n = chrono::Utc::now();
                let token = encode(
                    &Header::default(),
                    &{
                        let mut gen_hash = std::collections::HashMap::new();
                        gen_hash.insert("email", user.email.clone());
                        gen_hash.insert("id", user.id.clone().unwrap());
                        gen_hash.insert("date", date_n.to_rfc3339());
                        gen_hash
                    },
                    &EncodingKey::from_secret("secret".as_ref()),
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
