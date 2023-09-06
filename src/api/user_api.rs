#![allow(unused)]
use crate::repository::mongodb_repo::DB;
use crate::todo_struct::todo;
use actix_web::{
    delete, get,
    http::{self, header::HttpDate},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};
use futures::future::ok;
use mongodb::bson::oid::ObjectId;
#[get("/hey")]
async fn hey() -> HttpResponse {
    HttpResponse::Ok().body("HELLO_WORLD")
}

#[post("/todo")]
pub async fn create_todo(db: Data<DB>, new_todo: Json<todo>) -> HttpResponse {
    println!("create_todo called");
    println!("new todo data provided{:?}", new_todo);
    let data = todo {
        id: None,
        // title: new_todo.title.to_owned(),
        // description: new_todo.description.to_owned(),
        title: new_todo.title.to_owned(),
        description: new_todo.description.to_owned(),
    };
    let created = db.create_todo(data).await;
    match created {
        Ok(created) => HttpResponse::Ok().json(created),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

    // HttpResponse::Ok().body("create todo here")
}
#[get("/todo/{id}")]
pub async fn getsingle(db: Data<DB>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let todo = db.getone(&id).await;
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[get("/todo")]
pub async fn get_all(db: Data<DB>) -> HttpResponse {
    let todos = db.get_todos().await;
    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/todo/{id}")]
pub async fn delete_todo(db: Data<DB>, path: Path<String>) -> HttpResponse {
    let id: String = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid Id");
    }
    let deleted = db.delete_todo(&id).await;
    match deleted {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("todo successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("todo with specified id not found");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[put("/todo/{id}")]
pub async fn update_todo(db: Data<DB>, path: Path<String>, new_todo: Json<todo>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid Id");
    };
    let data = todo {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_todo.title.to_owned(),
        description: new_todo.description.to_owned(),
    };
    let update_result = db.update_todo(&id, data).await;
    match update_result {
        Ok(updated) => {
            if updated.matched_count == 1 {
                return HttpResponse::Ok().json("todo succsessfully updated");
            } else {
                return HttpResponse::InternalServerError()
                    .json("no specified todo found with that id");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
