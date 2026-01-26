use axum::{
    Extension, Router,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
};
use axum_ws_live::{ChatState, ws_handler};
use rust_embed::Embed;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Embed)]
#[folder = "my-app/dist/"]
struct Assets;

struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: AsRef<str>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.as_ref();
        // 建议去掉开头的 / 以匹配嵌入文件的路径
        let path = path.trim_start_matches('/');
        match Assets::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                axum::response::Response::builder()
                    .header(axum::http::header::CONTENT_TYPE, mime.as_ref())
                    .body(axum::body::Body::from(content.data))
                    .unwrap_or_else(|_| {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Error").into_response()
                    })
            }
            None => (StatusCode::NOT_FOUND, "File Not Found").into_response(),
        }
    }
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
        .route("/ws", get(ws_handler).layer(Extension(ChatState::new())))
        .fallback(get(state_handler));

    tracing::info!("启动 Todo 服务器在 http://localhost:3000");
    // 注意: 这里我们绑定到 0.0.0.0，这样可以接受来自任何 IP 地址的连接
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;
}

async fn state_handler(url: Uri) -> impl IntoResponse {
    StaticFile(url.path().to_string())
}

async fn index_handler() -> impl IntoResponse {
    state_handler("/index.html".parse().unwrap()).await
}
