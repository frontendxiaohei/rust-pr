use axum::{extract::State, routing::{post, Router}, Json};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use axum::http::StatusCode;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiResponse {
    code: u16,
    msg: String,
}

type UserDb = Arc<Mutex<HashMap<String, String>>>;

async fn register(
    State(db): State<UserDb>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    let mut db = db.lock().unwrap();
    if db.contains_key(&payload.username) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse { code: 400, msg: "用户名已存在".to_string() }),
        );
    }
    db.insert(payload.username, payload.password);
    (
        StatusCode::OK,
        Json(ApiResponse { code: 200, msg: "注册成功".to_string() }),
    )
}

async fn login(
    State(db): State<UserDb>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    let db = db.lock().unwrap();
    match db.get(&payload.username) {
        Some(pw) if pw == &payload.password => (
            StatusCode::OK,
            Json(ApiResponse { code: 200, msg: "登录成功".to_string() }),
        ),
        Some(_) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse { code: 401, msg: "密码错误".to_string() }),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse { code: 404, msg: "用户不存在".to_string() }),
        ),
    }
}

#[tokio::main]
async fn main() {
    let db: UserDb = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(db);
    println!("服务已启动: http://127.0.0.1:3000");


    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}