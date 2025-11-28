use std::sync::{Arc, Mutex};

use poem::{EndpointExt, Route, Server, get, listener::TcpListener, post};

use store::store::Store;

use crate::routes::{
    user::create_user,
    website::{create_website, get_website},
};

pub mod request_inputs;
pub mod request_outputs;
pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let s = Arc::new(Mutex::new(Store::new().unwrap()));

    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(create_user))
        .data(s);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
