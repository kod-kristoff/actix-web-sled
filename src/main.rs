use actix_web::{delete, get, http, put, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sled_extensions::bincode::Tree;
use std::io;

#[derive(Clone)]
struct Database {
    users: Tree<User>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    favorite_food: String,
}



type EndpointResult<T> = Result<T, ServerError>;

#[get("/users/{username}")]
async fn get_person(username: web::Path<String>, db: web::Data<Database>) -> EndpointResult<HttpResponse> {
    let user = db.users.get(username.as_bytes())?.ok_or_else(|| ServerError::NotFound)?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users")]
async fn put_person(user: web::Json<User>, db: web::Data<Database>) -> EndpointResult<HttpResponse> {
    db.users.insert(user.username.as_bytes(), user.clone())?;
    Ok(HttpResponse::Ok().json(user.0))
}

#[delete("/users/{username}")]
async fn delete_person(username: web::Path<String>, db: web::Data<Database>) -> EndpointResult<HttpResponse> {
    let user = db.users.remove(username.as_bytes())?.ok_or_else(|| ServerError::NotFound)?;
    Ok(HttpResponse::Ok().json(user))
}

#[actix_web::main]
async fn main() -> Result<(), io::Error>  {
    use actix_web::{App, HttpServer};
    use sled_extensions::DbExt;

    let db = sled_extensions::Config::default()
        .path("./sle_data")
        .open()
        .expect("Failed to open sled db");

    let database = Database {
        users: db.open_bincode_tree("users").unwrap(),
    };

    HttpServer::new(move || {
        App::new()
            .data(database.clone())
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
    #[error("sled db error")]
    SledError(#[from] sled_extensions::Error),
    #[error("not found")]
    NotFound,
    #[error("io error")]
    IoError(#[from] io::Error),
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        use actix_web::dev::HttpResponseBuilder;
        use actix_web::http::header;

        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> http::StatusCode {
        use actix_web::http::StatusCode;

        match *self {
            ServerError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
