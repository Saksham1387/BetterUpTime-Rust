use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOuput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserOuput {
    pub id: String,
    pub username: String,
    pub password: String,
}
