use actix::prelude::*;
use actix::{Actor, Addr, Context, Handler, Recipient, Message, StreamHandler};
use actix_web_actors::ws;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, Error};
// use actix_web::web::Payload;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, MySql, mysql::MySqlPoolOptions};
// MySqlPool
use actix_cors::Cors;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};


#[derive(Clone)]
struct AppState {
    // pool: sqlx::Pool<sqlx::MySql>,
    chat_server: Addr<ChatServer>,
    pool: sqlx::MySqlPool,
}

#[derive(Serialize, Deserialize, Debug)]
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


async fn register(data: web::Data<AppState>, register_req: web::Json<RegisterRequest>) -> impl Responder {
    println!("Registering user: {:?}", register_req.name);

    let hashed_password = match hash(&register_req.password, DEFAULT_COST) {
        Ok(hp) => {
            println!("Password hashed successfully");
            hp
        },
        Err(e) => {
            eprintln!("Password hashing failed: {:?}", e);
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

    let created_at_str = user.created_at.to_rfc3339();

    println!("User struct: {:?}", user);

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
                    name: user.name.clone(),
                    email: user.email.clone(),
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



struct WebSocketConnection {
    server: Addr<ChatServer>,
    client_id: Uuid,
}

impl WebSocketConnection {
    pub fn new(server: Addr<ChatServer>, client_id: Uuid) -> Self {
        Self { server, client_id }
    }
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection started for client: {}", self.client_id);

        let addr = ctx.address();
        self.server.do_send(Connect {
            id: self.client_id,
            addr, 
        });
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connection stopped for client: {}", self.client_id);

        self.server.do_send(Disconnect {
            id: self.client_id,
        });
    }
}

impl Handler<ChatMessage> for WebSocketConnection {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        let text = serde_json::to_string(&msg).unwrap_or_else(|_| "Error serializing message".to_string());
        ctx.text(text);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);

                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    self.server.do_send(client_msg);
                } else {
                    eprintln!("Invalid message format: {}", text);
                }
            }
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket connection closed: {:?}", reason);
                ctx.stop();
            }
            Err(err) => {
                eprintln!("WebSocket error: {:?}", err);
                ctx.stop();
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Message)] 
#[rtype(result = "()")] 
struct ChatMessage {
    id: Uuid,
    sender: String,
    text: String,
}

#[derive(Clone, Debug, Deserialize, Message)]
#[rtype(result = "()")]
struct ClientMessage {
    sender: String,
    text: String,
}

#[derive(Message)]
#[rtype(result = "()")] 
struct Connect {
    id: Uuid,
    addr: Addr<WebSocketConnection>, 
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    id: Uuid,
}

struct ChatServer {
    clients: HashMap<Uuid, Recipient<ChatMessage>>,
    pool: Pool<MySql>,
}

impl ChatServer {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            clients: HashMap::new(),
            pool,
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        let recipient = msg.addr.recipient::<ChatMessage>();
        self.clients.insert(msg.id, recipient);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        self.clients.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) {
        let db_pool = self.pool.clone();
        let message_id = Uuid::new_v4();
        let sender_for_db = msg.sender.clone();
        let text_for_db = msg.text.clone();

       println!("{} {} {}", message_id.to_string(), sender_for_db, text_for_db);

        actix::spawn(async move {
            let query_result = sqlx::query!(
                "INSERT INTO msgs (id, sender, text) VALUES (?, ?, ?)",
                message_id.to_string(),
                sender_for_db,
                text_for_db
            )
            .execute(&db_pool)
            .await;

            if let Err(e) = query_result {
                eprintln!("Database error: {:?}", e);
            } else {
                println!("Message inserted successfully: {:?}", text_for_db);
            }
        });

        for (_, client) in &self.clients {
            let _ = client.do_send(ChatMessage {
                id: message_id,
                sender: msg.sender.clone(),
                text: msg.text.clone(),
            });
        }
    }
}


async fn websocket_connection(
    req: HttpRequest, 
    stream: web::Payload,
    server: web::Data<Addr<ChatServer>>
) -> impl Responder { // Result<HttpResponse, Error>
    let client_id = Uuid::new_v4();
    println!("WebSocket connection established for client: {}", client_id);

    let websocket = WebSocketConnection::new(server.get_ref().clone(), client_id);

    match ws::start(websocket, &req, stream) {
        Ok(res) => {
            println!("WebSocket connection successful for client: {}", client_id);
            res
        }
        Err(e) => {
            eprintln!("Failed to start WebSocket: {:?}", e);
            HttpResponse::InternalServerError().finish() 
        }
    }
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { //
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5) 
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");
    println!("Connected to MySQL database successfully!");

    let chat_server = ChatServer::new(pool.clone()).start();
    println!("Starting WebSocket server on http://127.0.0.1:8080/");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app_state = AppState {
            chat_server: chat_server.clone(),
            pool: pool.clone(),
        };

        App::new()
            .app_data(web::Data::new(app_state))
            .wrap(cors)
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/ws", web::get().to(websocket_connection))
    }).bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}
