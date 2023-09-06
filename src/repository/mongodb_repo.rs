#![allow(unused)]
use std::env;
extern crate dotenv;
use crate::todo_struct::todo;
use actix_web::HttpResponse;
use dotenv::{dotenv, Error};
use futures::future::ok;
use futures::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, document};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Client, Collection};
const DB_NAME: &str = "rust-todo";
const COLL: &str = "todos";
const ID: &str = "_id";
const TITLE: &str = "title";
const DESCRIPTION: &str = "description";
// #[derive(Clone, Debug)]
pub struct DB {
    pub col: Collection<todo>,
}
impl DB {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("error loading var"),
        };
        println!("{}", uri);
        let client = Client::with_uri_str(&uri)
            .await
            .expect("error connecting to the database");
        let db = client.database(DB_NAME);
        let col = db.collection("todos");
        Self { col }
    }
    pub async fn create_todo(&self, todo: todo) -> Result<InsertOneResult, Error> {
        let new_doc = todo {
            id: None,
            title: todo.title,
            description: todo.description,
        };
        let todo_result = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("error creating todo");
        Ok(todo_result)
    }
    pub async fn get_todos(&self) -> Result<Vec<todo>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of todos");
        let mut todos: Vec<todo> = Vec::new();

        while let Some(todo) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error maping through cursor")
        {
            todos.push(todo)
        }
        Ok(todos)
    }
    pub async fn update_todo(&self, id: &String, new_todo: todo) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":obj_id};
        let new_doc = doc! {
            "$set":{
                "title":new_todo.title,
                "description":new_todo.description,
            }
        };

        let updated = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("error updating the todo");
        Ok(updated)
    }
    pub async fn delete_todo(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let deleted = self
            .col
            .delete_one(doc! {"_id":obj_id}, None)
            .await
            .ok()
            .expect("error deleting user");
        Ok(deleted)
    }
    pub async fn getone(&self, id: &String) -> Result<todo, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {
            "_id":obj_id,
        };
        let todo = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("error getting users's details");
        Ok(todo.unwrap())
    }
}
