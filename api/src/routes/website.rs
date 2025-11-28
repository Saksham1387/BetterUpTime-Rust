use std::sync::{Arc, Mutex};

use poem::{
    handler,
    web::{Data, Json},
};

use crate::{
    request_inputs::{CreateWebsite, GetWebsite},
    request_outputs::{CreateWebsiteOuput, GetWebsiteOuput},
};
use store::store::Store;

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsite>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOuput> {
    let url = data.url;
    let mut locked_s = s.lock().unwrap();
    let db_result = locked_s
        .create_website(String::from("4116c8aa-3aa3-47ce-bbef-07db6a105c51"), url)
        .unwrap();
    let response = CreateWebsiteOuput { id: db_result.id };
    Json(response)
}

#[handler]
pub fn get_website(
    Json(data): Json<GetWebsite>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOuput> {
    let id = data.id;
    let mut locked_s = s.lock().unwrap();
    let db_result = locked_s.get_website(id).unwrap();
    let response = GetWebsiteOuput {
        id: db_result.id,
        url: db_result.url,
        user_id: db_result.user_id,
    };
    Json(response)
}
