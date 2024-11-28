#[macro_use]
extern crate rocket;

use rocket::tokio::sync::broadcast::{chanel};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use uuid::Uuid;
use std::sync::{Mutex, MutexGuard};
use std::fs::File;
use std::io::Write;
use chrono::{Utc, DateTime};

// #[derive(Debug, Serialize, Deserialize)]
// struct User {
//     id: Uuid,
//     name: String,
//     email: String,
//     password: String
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Message {
//     id: Uuid,
//     author: Uuid,
//     text: String,
//     created_at: DateTime<Utc>
// }

// #[derive(Deserialize)]
// struct CreateMessage {
//     author: Uuid,
//     text: String
// }


// struct AppState {
//     msgs_list: Mutex<Vec<TodoItem>>,
// }

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::
    .mount("/hello", routes![world])
}




// // SAVE msgs to file
// async fn save_msgs(data: web::Data<AppState>) -> impl Responder {
//     let todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();

//     match save_todos_to_file(&todos) {
//         Ok(_) => HttpResponse::Ok().body("Todos saved successfully"),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Failed to save todos: {}", e)),
//     }
// }

// // Helper function to save msgs
// fn save_msgs_to_file(todos: &Vec<Message>) -> std::io::Result<()> {
//     let dir = std::path::Path::new("data");
//     let path = dir.join("msgs.json");

//     if !dir.exists() {
//         std::fs::create_dir_all(&dir)?; 
//     }

//     let file_result = std::fs::File::create(&path);

//     match file_result {
//         Ok(mut file) => {
//             match serde_json::to_string_pretty(&todos) {
//                 Ok(todos_json) => {
//                     match file.write_all(todos_json.as_bytes()) {
//                         Ok(_) => Ok(()),
//                         Err(e) => {
//                             eprintln!("Error writing to file: {}", e);
//                             Err(e)
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error serializing todos to JSON: {}", e);
//                     Err(std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed"))
//                 }
//             }
//         }
//         Err(e) => {
//             eprintln!("Error creating or opening the file: {}", e);
//             Err(e)
//         }
//     }
// }

// // LOAD msgs 
// async fn load_msgs(data: web::Data<AppState>) -> impl Responder {
//     match load_todos_from_file() {
//         Ok(saved_todos) => {
//             let mut todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();
//             *todos = saved_todos; 
//             HttpResponse::Ok().json(&*todos) 
//         }
//         Err(e) => HttpResponse::InternalServerError().body(format!("Failed to load todos: {}", e)),
//     }
// }

// // Helper function to read msgs
// fn load_msgs_from_file() -> std::io::Result<Vec<Message>> {
//     let path = std::path::Path::new("data/msgs.json");

//     if !path.exists() {
//         return Ok(Vec::new()); 
//     }

//     let file = File::open(&path)?;
//     let new_todos: Vec<TodoItem> = from_reader(file)?;
//     Ok(new_todos)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let app_state = web::Data::new(AppState{
//         todos_list: Mutex::new(Vec::new()),
//     });
//     HttpServer::new(move || {
//         let cors = Cors::default()
//                         .allow_any_origin()
//                         .allow_any_method()
//                         .allow_any_header()
//                         .max_age(3600);

//         App::new().app_data(app_state.clone()).wrap(cors)
//             .route("/todos", web::get().to(get_todos))
//             .route("/todos", web::post().to(add_todo))
//             .route("/todos/{id}", web::put().to(update_todo))
//             .route("/todos/{id}", web::delete().to(delete_todo))
//             .route("/todos/save", web::post().to(save_todos))
//             .route("/todos/load", web::get().to(load_todos))
//     }).bind("127.0.0.1:8080")?.run().await

// }