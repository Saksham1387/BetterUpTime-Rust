use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsite {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsite {
    pub id: String,
}
