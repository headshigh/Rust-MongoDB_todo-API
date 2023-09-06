mod api;
mod repository;
mod todo_struct;
use crate::api::user_api::{create_todo, delete_todo, get_all, getsingle, hey, update_todo};
use actix_web::{get, web, web::Data, App, HttpServer, Responder};
use repository::mongodb_repo::DB;
#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    //env_logger = "0.10.0"
    let db = DB::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(index)
            .service(create_todo)
            .service(get_all)
            .service(getsingle)
            .service(delete_todo)
            .service(update_todo)
            // .service(hello)
            .service(hey)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
