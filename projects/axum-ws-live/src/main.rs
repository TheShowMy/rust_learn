use axum::{Extension, Router, response::Html, routing::get};
use axum_ws_live::{ChatState, ws_handler};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
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
        .route("/ws", get(ws_handler).layer(Extension(ChatState::new())));

    tracing::info!("启动 Todo 服务器在 http://localhost:3000");
    // 注意: 这里我们绑定到 0.0.0.0，这样可以接受来自任何 IP 地址的连接
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;
}

async fn index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
