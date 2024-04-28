use bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub password: String,
    pub token: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct UserLoginResponse {
    pub token: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct LogoutRequest {
    pub email: String,
}
