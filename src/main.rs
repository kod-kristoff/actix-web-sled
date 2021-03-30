use actix_web::{delete, get, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    favorite_food: String,
}

type EndpointResult<T> = Result<T, ServerError>;

#[get("/users/{username}")]
async fn get_person(username: web::Path<String>) -> EndpointResult<HttpResponse> {
    todo!()
}

#[put("/users")]
async fn put_person(user: web::Json<User>) -> EndpointResult<HttpResponse> {
    todo!()
}

#[delete("/users/{username}")]
async fn delete_person(username: web::Path<String>) -> EndpointResult<HttpResponse> {
    todo!()
}

#[actix_web::main]
async fn main() -> io::Result<()>  {
    use actix_web::{App, HttpServer};

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .service(get_person)
                    .service(delete_person)
                    .service(put_person)
            )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("not found")]
    NotFound,
}

impl actix_web::error::ResponseError for ServerError {}
