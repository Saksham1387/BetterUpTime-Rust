use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOuput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserOuput {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsiteOuput {
    pub id: String,
    pub url: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SigninOutput {
    pub jwt: String,
}
