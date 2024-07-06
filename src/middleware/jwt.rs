use crate::{db::mongodb::get_db, models::users::User};
use bson::doc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    email: String,
    id: String,
    date: String,
    exp: usize,
}

pub async fn validate_token(token: &str) -> Result<bool, bool> {
    let token_prop = token.to_string();
    let token_decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("JWT_SECRET".as_ref()),
        &Validation::new(Algorithm::HS256),
    );

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

            let filter = doc! { "id": claims.id, "email": claims.email };
            let user = collection.find_one(filter, None).await;
            println!("{:?}", user);

            match user {
                Ok(Some(user)) => {
                    if let Some(token) = user.token {
                        if token == token_prop {
                            return Ok(true);
                        } else {
                            return Err(false);
                        }
                    } else {
                        return Err(false);
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
}
