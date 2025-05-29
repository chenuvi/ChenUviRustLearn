use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

// 用户结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    fn new(id: u32, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

// 创建用户的请求体
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

// 更新用户的请求体
#[derive(Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

// 应用状态
type AppState = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    const ADDR: &str = "0.0.0.0:3000";

    // 初始化用户数据
    let users = Arc::new(Mutex::new(vec![
        User::new(1, "张三".to_string(), "zhangsan@example.com".to_string()),
        User::new(2, "李四".to_string(), "lisi@example.com".to_string()),
        User::new(3, "王五".to_string(), "wangwu@example.com".to_string()),
    ]));

    // 创建路由
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/hello/{name}", get(hello))
        .with_state(users);

    // 绑定地址和端口
    let listener = TcpListener::bind(ADDR).await.unwrap();

    println!("🚀 服务器启动成功！");
    println!("📍 地址: http://{}", ADDR);
    println!("📋 可用端点:");
    println!("   GET    / - 主页");
    println!("   GET    /health - 健康检查");
    println!("   GET    /api/users - 获取用户列表");
    println!("   POST   /api/users - 创建用户");
    println!("   GET    /api/users/{{id}} - 获取单个用户");
    println!("   PUT    /api/users/{{id}} - 更新用户");
    println!("   DELETE /api/users/{{id}} - 删除用户");
    println!("   GET    /hello/{{name}} - 个性化问候");

    // 启动服务器
    axum::serve(listener, app).await.unwrap();
}

// 根路由处理器
async fn root() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Axum Web 服务器</title>
            <meta charset="utf-8">
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
                .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
                h1 { color: #333; text-align: center; }
                .endpoint { background: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 4px solid #007bff; }
                .method { font-weight: bold; color: #007bff; }
                .method.post { color: #28a745; }
                .method.put { color: #ffc107; }
                .method.delete { color: #dc3545; }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🦀 Axum Web 服务器</h1>
                <p>欢迎使用 Rust + Axum 构建的 Web 服务器！支持完整的 CRUD 操作。</p>
                
                <h2>📋 API 端点</h2>
                <div class="endpoint">
                    <span class="method">GET</span> /health - 健康检查
                </div>
                <div class="endpoint">
                    <span class="method">GET</span> /api/users - 获取所有用户
                </div>
                <div class="endpoint">
                    <span class="method post">POST</span> /api/users - 创建新用户
                </div>
                <div class="endpoint">
                    <span class="method">GET</span> /api/users/{id} - 获取单个用户
                </div>
                <div class="endpoint">
                    <span class="method put">PUT</span> /api/users/{id} - 更新用户
                </div>
                <div class="endpoint">
                    <span class="method delete">DELETE</span> /api/users/{id} - 删除用户
                </div>
                <div class="endpoint">
                    <span class="method">GET</span> /hello/{name} - 个性化问候
                </div>
            </div>
        </body>
        </html>
    "#,
    )
}

// 健康检查
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "message": "服务器运行正常 ✅"
    }))
}

// 获取所有用户 - 优化：使用 iter().cloned() 而不是手动构造
async fn get_users(State(users): State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let users_guard = users
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let users_vec: Vec<User> = users_guard.iter().cloned().collect();
    Ok(Json(users_vec))
}

// 创建用户 - 避免额外的克隆，直接返回新创建的用户
async fn create_user(
    State(users): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let mut users_guard = users
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 生成新的 ID
    let new_id = users_guard.iter().map(|u| u.id).max().unwrap_or(0) + 1;

    let new_user = User::new(new_id, payload.name, payload.email);
    users_guard.push(new_user.clone());

    Ok((StatusCode::CREATED, Json(new_user)))
}

// 获取单个用户 - 只克隆找到的用户
async fn get_user(
    Path(user_id): Path<u32>,
    State(users): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users_guard = users
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match users_guard.iter().find(|user| user.id == user_id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// 更新用户 - 原地修改后克隆返回
async fn update_user(
    Path(user_id): Path<u32>,
    State(users): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let mut users_guard = users
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match users_guard.iter_mut().find(|user| user.id == user_id) {
        Some(user) => {
            if let Some(name) = payload.name {
                user.name = name;
            }
            if let Some(email) = payload.email {
                user.email = email;
            }

            Ok(Json(user.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// 删除用户 - 使用 retain，这是最高效的删除方式
async fn delete_user(
    Path(user_id): Path<u32>,
    State(users): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut users_guard = users
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let initial_len = users_guard.len();
    users_guard.retain(|user| user.id != user_id);

    if users_guard.len() < initial_len {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// 问候处理器
async fn hello(Path(name): Path<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": format!("你好, {}! 欢迎使用 Axum! ��", name)
    }))
}
