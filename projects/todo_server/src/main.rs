use axum::{Json, Router, http::StatusCode, response::Html, routing::get};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTodo {
    title: String,
}

#[tokio::main]
async fn main() {
    // 初始化日志记录
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 创建路由器
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/todos", get(todos_handler).post(create_todos_handler));
    tracing::info!("启动 Todo 服务器在 http://localhost:3000");
    // 注意: 这里我们绑定到 0.0.0.0，这样可以接受来自任何 IP 地址的连接
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Welcome to the Todo Server</h1>")
}

async fn todos_handler() -> Json<Vec<Todo>> {
    tracing::info!("调用 todos_handler");
    let todos = vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Build a web server".to_string(),
            completed: true,
        },
    ];
    Json(todos)
}

async fn create_todos_handler(Json(todo): Json<CreateTodo>) -> StatusCode {
    tracing::info!("Creating todo: {:?}", todo);
    StatusCode::CREATED
}
