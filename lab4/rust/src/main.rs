use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};
use actix_cors::Cors;
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;
use chrono::Utc;
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    token: Option<String>,
}

#[derive(Deserialize)]
struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

const JWT_SECRET: &str = "mysecret";

// async fn register_user(
//     pool: web::Data<Pool<MySql>>,
//     user: web::Json<RegisterRequest>,
// ) -> impl Responder {
//     let user_id = Uuid::new_v4();
//     let hashed_password = hash(&user.password, bcrypt::DEFAULT_COST).unwrap();

//     let result = sqlx::query!(
//         "INSERT INTO users (id, name, email, password) VALUES (?, ?, ?, ?)",
//         user_id.to_string(),
//         user.name,
//         user.email,
//         hashed_password
//     )
//     .execute(pool.get_ref())
//     .await;

//     match result {
//         Ok(_) => HttpResponse::Ok().json("User registered successfully"),
//         Err(err) => {
//             eprintln!("Error: {}", err);
//             HttpResponse::InternalServerError().json("Failed to register user")
//         }
//     }
// }

// async fn login_user(
//     pool: web::Data<Pool<MySql>>,
//     credentials: web::Json<LoginRequest>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let user = sqlx::query_as(
//         User,
//         "SELECT id as `id: Uuid`, name, email, password, CAST(NULL AS CHAR(255)) as token FROM users WHERE email = ?",
//         credentials.email
//     )
//     .fetch_optional(pool.get_ref())
//     .await
//     .map_err(|e| {
//         eprintln!("Query Error: {}", e);
//         actix_web::error::ErrorInternalServerError("Database query failed")
//     })?;

//     if let Some(user) = user {
//         if verify(&credentials.password, &user.password).unwrap() {
//             let expiration = Utc::now()
//                 .checked_add_signed(chrono::Duration::hours(1))
//                 .expect("valid timestamp")
//                 .timestamp() as usize;

//             let claims = Claims {
//                 sub: user.id,
//                 exp: expiration,
//             };

//             let token = encode(
//                 &Header::default(),
//                 &claims,
//                 &EncodingKey::from_secret(JWT_SECRET.as_ref()),
//             )
//             .unwrap();

//             return HttpResponse::Ok().json(token);
//         }
//     }

//     Ok(HttpResponse::Unauthorized().json("Invalid credentials"))
// }

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5) 
        .connect(&database_url)
        .await?;

    println!("Connected to MySQL database successfully!");

    let rows = sqlx::query!("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        println!("ID: {}, Name: {}", row.id, row.name);
    }

    Ok(())
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