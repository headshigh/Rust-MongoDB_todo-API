# Rust-MongoDB_todo-API
Small project that showcases the use of Actixweb and MongoDB with Rust  

# Setup
Get a connection string from mongodb atlas, create a database named as ```rust-todo``` and a collection named as ```todos```. Place your connection string in .env file.

# Run 
```bash 
cargo run
```
Runs the server on  http://localhost:8080  

You may use CRUD operations like this  

Create Todo :  
 
```Post``` request on ```localhost:8080/todo``` with body as
``` bash
{
"title":"todo title",
"description":"todo description"
}
```
Get All Todos:   

```GET``` request on ```localhost:8080/todo```   

Update a Todo:  

```PUT```  request on ```localhost:8080/todo/<your todo id>``` with body as
``` bash
{
"title":"updated title",
"description":"updated description"
}
```
Delete a Todo:  

```DELETE```  request on ```localhost:8080/todo/<your todo id>``` 
