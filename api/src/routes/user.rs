use std::sync::{Arc, Mutex};

use poem::{
    handler,
    web::{Data, Json},
};

use crate::{request_inputs::CreateUser, request_outputs::CreateUserOuput};
use store::store::Store;

#[handler]
pub fn create_user(
    Json(data): Json<CreateUser>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOuput> {
    let mut locked_s = s.lock().unwrap();

    let pass = data.password;
    let username = data.username;
    let db_result = locked_s.signup(pass, username).unwrap();

    let response = CreateUserOuput { id: db_result };

    Json(response)
}
