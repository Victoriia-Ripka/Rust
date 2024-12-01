use actix::prelude::*;
use actix::{Actor, Addr, Context, Handler, Recipient, Message};
use actix_web_actors::ws;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, Error};
use actix_web::web::Payload;
use serde::{Deserialize, Serialize};
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
//  Pool, MySql, mysql::MySqlPool
use actix_cors::Cors;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;


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

#[derive(Serialize)]
struct LoginResponse {
    name: String,
    email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Message)] 
#[rtype(result = "()")] 
struct ChatMessage {
    sender: String,
    message: String,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
struct ClientMessage {
    id: Uuid,
    message: ChatMessage,
}

#[derive(Message)]
#[rtype(result = "()")] 
struct Connect {
    id: Uuid,
    addr: Recipient<ChatMessage>, 
}

struct WebSocketConnection;

pub struct ChatServer {
    clients: HashMap<Uuid, Recipient<ChatMessage>>,
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::Pool<sqlx::MySql>,
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received WebSocket message: {}", text);
                ctx.text(text);
            }
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket closed: {:?}", reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        for (id, client) in &self.clients {
            if *id != msg.id {
                let _ = client.do_send(msg.message.clone());
            }
        }
    }
}

async fn register(data: web::Data<AppState>, register_req: web::Json<RegisterRequest>) -> impl Responder {
    let hashed_password = match hash(&register_req.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => {
            return HttpResponse::InternalServerError().json("Failed to hash password");
        }
    };

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


async fn login(data: web::Data<AppState>, login_req: web::Json<LoginRequest>) -> impl Responder {
    let query_result = sqlx::query!(
        "SELECT name, email, password FROM users WHERE email = ?",
        login_req.email
    )
   .fetch_one(&data.pool)
   .await;

    match query_result {
        Ok(user) => {

            let is_valid_password = verify(&login_req.password, &user.password).unwrap_or(false);

            if is_valid_password {
                let user_response = LoginResponse {
                    name: user.name,
                    email: user.email,
                };
                HttpResponse::Ok().json(serde_json::json!({"message": "Login successful", "data": user_response}))
            } else {
                HttpResponse::Unauthorized().body("Invalid email or password")
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            HttpResponse::InternalServerError().json("Failed to authenticate user")
        }
    }
}


async fn websocket_connection(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocketConnection {}, &req, stream)
}

// async fn post_message(
//     server: web::Data<Addr<ChatServer>>,
//     message: web::Json<ChatMessage>,
// ) -> impl Responder {
//     server.do_send(ClientMessage {
//         id: Uuid::new_v4(),
//         message: message.into_inner(),
//     });
//     HttpResponse::Ok().json("status: Message sent")
// }

// #[tokio::main]
#[actix_web::main]
// async fn main() -> Result<(), sqlx::Error> {
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5) 
        .connect(&database_url)
        .await?;

    println!("Connected to MySQL database successfully!");

    let chat_server = ChatServer::new().start();

    println!("Starting WebSocket server on http://127.0.0.1:8080/");

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
            .route("/login", web::post().to(login))
            .route("/ws", web::get().to(websocket_connection))
    }).bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}
