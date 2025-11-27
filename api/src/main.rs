use poem::{
    EndpointExt, Route, Server, get, handler,
    listener::TcpListener,
    middleware::Tracing,
    post,
    web::{Json, Path},
};

use crate::{
    request_inputs::{CreateUser, CreateWebsite},
    request_outputs::{CreateUserOuput, CreateWebsiteOuput},
};

pub mod request_inputs;
pub mod request_outputs;

use store::store::Store;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {website_id}")
}

#[handler]
fn create_website(Json(data): Json<CreateWebsite>) -> Json<CreateWebsiteOuput> {
    let url = data.url;
    let response = CreateWebsiteOuput {
        id: String::from("iowejur9050u235io"),
    };

    let s = Store::default();

    Json(response)
}

#[handler]
fn create_user(Json(data): Json<CreateUser>) -> Json<CreateUserOuput> {
    let pass = data.password;
    let username = data.username;
    let s = Store::default();
    let db_result = s::sing
    let s = Store::default();

    Json(response)
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post());
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
