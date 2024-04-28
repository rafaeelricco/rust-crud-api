use bson::doc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{db::mongodb::get_db, models::users::User};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    id: String,
    date: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, bool> {
    info!("Iniciando validação do token...");

    let token_decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("JWT_SECRET".as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    info!("Token decodificado: {:?}", token_decoded);

    match token_decoded {
        Ok(TokenData { claims, .. }) => {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards.")
                .as_secs() as usize;

            if current_time >= claims.exp {
                info!("Token expirado.");
                return Err(false);
            }

            let db = get_db().await;
            let collection = db.collection::<User>("users");
            info!("Buscando usuário no banco de dados...");

            let filter = doc! { "id": claims.id };
            let user = collection.find_one(filter, None).await;

            match user {
                Ok(Some(user)) => {
                    info!("Usuário encontrado: {:?}", user.email);
                    if user.token == Some(token.to_string()) {
                        return Ok(true);
                    }
                }
                Ok(None) => {
                    return Err(false);
                }
                Err(e) => {
                    info!("{:?}", e);
                    return Err(false);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(false);
        }
    }

    Err(false)
}
