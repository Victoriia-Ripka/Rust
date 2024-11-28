use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql, mysql::MySqlPool};
use actix_cors::Cors;
use bcrypt::{hash, verify, DEFAULT_COST };
// use jsonwebtoken::{encode, Header, EncodingKey};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;

#[derive(Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
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

pub struct AppState {
    pub pool: MySqlPool,
}

// const JWT_SECRET: &str = "mysecret";

async fn register(data: web::Data<AppState>, register_req: web::Json<RegisterRequest>) -> impl Responder {
    let hashed_password = match hash(&register_req.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => {
            return HttpResponse::InternalServerError().json("Failed to hash password");
        }
    };
    println!("Registering user: {:?}", register_req.name);

    let user = User {
        id: Uuid::new_v4(),
        name: register_req.name.clone(),
        email: register_req.email.clone(),
        password: hashed_password,
        created_at: Utc::now(),
    };

    // Convert created_at to a string
    let created_at_str = user.created_at.to_rfc3339();

    let query_result = sqlx::query!(
        "INSERT INTO users (id, name, email, password, created_at) VALUES (?, ?, ?, ?, ?)",
        user.id.to_string(),
        user.name,
        user.email,
        user.password,
        created_at_str 
    )
    .execute(&data.pool)
    .await;

    match query_result {
        Ok(_) => {
            println!("User registered successfully: {:?}", user.name);
            HttpResponse::Created().json(&user)
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().json("Failed to register user")
        }
    }
}



#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5) 
        .connect(&database_url)
        .await?;

    println!("Connected to MySQL database successfully!");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(AppState { pool: pool.clone() }))
            .wrap(cors)
            .route("/register", web::post().to(register))
    }).bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}
