use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use uuid::Uuid;
use std::sync::{Mutex, MutexGuard};
use std::fs::File;
use std::io::Write;
use chrono::{Utc, DateTime};


#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct CreateTodoItem {
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct UpdateTodoItem {
    title: Option<String>,
    completed: Option<bool>,
}
struct AppState {
    todos_list: Mutex<Vec<TodoItem>>,
}

// READ
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

// CREATE
async fn add_todo(item: web::Json<CreateTodoItem>, data: web::Data<AppState>) -> impl Responder {
    let mut todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();
    let new_todo = TodoItem {
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: item.completed,
        created_at: Utc::now(),
    };
    todos.push(new_todo);
    HttpResponse::Ok().json(&*todos)
}

// UPDATE
async fn update_todo(path: web::Path<Uuid>, item: web::Json<UpdateTodoItem>, data: web::Data<AppState>,) -> impl Responder {
    let mut todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == *path) {
        if let Some(title) = &item.title {
            todo.title = title.clone();
        }
        if let Some(completed) = item.completed {
            todo.completed = completed;
        }
        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo item not found")
    }
}

// DELETE
async fn delete_todo(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let mut todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();
    if todos.iter().any(|t| t.id == *path) {
        todos.retain(|t| t.id != *path);
        HttpResponse::Ok().json(&*todos)
    } else {
        HttpResponse::NotFound().body("Todo item not found")
    }
}


// SAVE todos to file
async fn save_todos(data: web::Data<AppState>) -> impl Responder {
    let todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();

    match save_todos_to_file(&todos) {
        Ok(_) => HttpResponse::Ok().body("Todos saved successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to save todos: {}", e)),
    }
}

// Helper function to save todos
fn save_todos_to_file(todos: &Vec<TodoItem>) -> std::io::Result<()> {
    let dir = std::path::Path::new("tasks");
    let path = dir.join("tasks.json");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?; 
    }

    let file_result = std::fs::File::create(&path);

    match file_result {
        Ok(mut file) => {
            match serde_json::to_string_pretty(&todos) {
                Ok(todos_json) => {
                    match file.write_all(todos_json.as_bytes()) {
                        Ok(_) => Ok(()),
                        Err(e) => {
                            eprintln!("Error writing to file: {}", e);
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error serializing todos to JSON: {}", e);
                    Err(std::io::Error::new(std::io::ErrorKind::Other, "Serialization failed"))
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating or opening the file: {}", e);
            Err(e)
        }
    }
}

// LOAD todos into todos_list
async fn load_todos(data: web::Data<AppState>) -> impl Responder {
    match load_todos_from_file() {
        Ok(saved_todos) => {
            let mut todos: MutexGuard<Vec<TodoItem>> = data.todos_list.lock().unwrap();
            *todos = saved_todos; 
            HttpResponse::Ok().json(&*todos) 
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to load todos: {}", e)),
    }
}

// Helper function to read todos
fn load_todos_from_file() -> std::io::Result<Vec<TodoItem>> {
    let path = std::path::Path::new("tasks/tasks.json");

    if !path.exists() {
        return Ok(Vec::new()); 
    }

    let file = File::open(&path)?;
    let new_todos: Vec<TodoItem> = from_reader(file)?;
    Ok(new_todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState{
        todos_list: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        let cors = Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .max_age(3600);

        App::new().app_data(app_state.clone()).wrap(cors)
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos/save", web::post().to(save_todos))
            .route("/todos/load", web::get().to(load_todos))
    }).bind("127.0.0.1:8080")?.run().await

}