use crate::{db::mongodb::get_db, models::users::User};
use actix_web::http::header;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web_httpauth::headers;
use bson::doc;
use chrono::Utc;
use futures::{Future, FutureExt};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    id: String,
    date: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, String> {
    info!("Iniciando validação do token...");

    let token_decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    info!("Token decodificado: {:?}", token_decoded);

    match token_decoded {
        Ok(TokenData { claims, .. }) => {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as usize;

            if current_time >= claims.exp {
                info!("Token exirado.");
                return Err("Token expirado".to_string());
            }

            let db = get_db().await;
            let collection = db.collection::<User>("users");
            info!("Buscando usuário no banco de dados...");

            let filter = doc! { "id": claims.id };
            let user = collection.find_one(filter, None).await;
            info!("Usuário encontrado: {:?}", user);

            match user {
                Ok(Some(user)) => {
                    if user.token == Some(token.to_string()) {
                        info!("Token válido.");
                        return Ok(true);
                    }
                }
                Ok(None) => {
                    return Err("Usuário não encontrado".to_string());
                }
                Err(e) => {
                    return Err(format!("Erro ao buscar usuário: {:?}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("Token inválido: {:?}", e));
        }
    }

    Err("Token inválido ou usuário não encontrado".to_string())
}

pub async fn auth_middleware(req: HttpRequest) -> Result<HttpRequest, Error> {
    let headers = req.headers();
    let auth_header = headers.get("Authorization");

    match auth_header {
        Some(auth_header) => {
            let token = auth_header.to_str().unwrap().replace("Bearer ", "");
            let token_valid = validate_token(&token).await;

            match token_valid {
                Ok(_) => Ok(req),
                Err(e) => {
                    info!("Erro ao validar token: {:?}", e);
                    Err(actix_web::error::ErrorUnauthorized("Token inválido"))
                }
            }
        }
        None => Err(actix_web::error::ErrorUnauthorized("Token não informado")),
    }
}

// use std::future::{ready, Ready};

// use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
// use futures_util::future::LocalBoxFuture;
// use futures_util::TryFutureExt;
// use std::task::{Context, Poll};

// // There are two steps in middleware processing.
// // 1. Middleware initialization, middleware factory gets called with
// //    next service in chain as parameter.
// // 2. Middleware's call method gets called with normal request.
// pub struct AuthMiddleware;

// // Middleware factory is `Transform` trait
// // `S` - type of the next service
// // `B` - type of response's body
// impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = ValidateTokenMiddleware<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;

//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(ValidateTokenMiddleware { service }))
//     }
// }

// pub struct ValidateTokenMiddleware<S> {
//     service: S,
// }

// impl<S, B> Service<ServiceRequest> for ValidateTokenMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

//     fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         info!("Middleware called");
//         let headers = req.headers().clone();
//         let auth_header_clone = headers.clone().get("Authorization");
//         let cloned_req = req.headers();

//         let fut = async move {
//             match auth_header_clone {
//                 Some(auth_header_clone) => {
//                     let token = auth_header_clone.to_str().unwrap().replace("Bearer ", "");
//                     let token_valid = validate_token(&token).await;

//                     match token_valid {
//                         Ok(_) => {
//                             info!("Token válido");
//                             // Se o token for válido, permite que a requisição prossiga
//                         }
//                         Err(e) => {
//                             info!("Erro ao validar token: {:?}", e);
//                             // Se o token não for válido, retorna um erro
//                             Err(actix_web::error::ErrorUnauthorized("Token inválido"))
//                         }
//                     }
//                 }
//                 None => Err(actix_web::error::ErrorUnauthorized("Token não informado")),
//             }
//         };

//         Box::pin(fut)
//     }
// }
