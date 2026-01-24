use std::sync::{Arc, RwLock, atomic::AtomicUsize};

use axum::{
    Extension, Json, Router,
    extract::FromRequestParts,
    http::{StatusCode, Uri, request::Parts},
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use jsonwebtoken as jwt;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const SECRET_KEY: &[u8] = b"TheShow";
const NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    user_id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Default, Clone)]
struct TodoStore {
    todos: Arc<RwLock<Vec<Todo>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTodo {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
    // exp: usize,
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = HttpError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // 从 Header 中提取 Token
        let TypedHeader(Authorization(auth_header)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| HttpError::Auth)?;

        let token = auth_header.token();

        // 验证 JWT
        let mut validation = jwt::Validation::default();
        validation.set_required_spec_claims::<String>(&[]);
        validation.validate_exp = false; // 如果你不想验证 exp 字段
        let token_data = jwt::decode::<Claims>(
            token,
            &jwt::DecodingKey::from_secret(SECRET_KEY),
            &validation,
        )
        .map_err(|e| {
            println!("err:{:?}", e);
            HttpError::Auth
        })?;

        Ok(token_data.claims)
    }
}

#[derive(Debug)]
enum HttpError {
    Auth,
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, message) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        (code, message).into_response()
    }
}

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
    let store = TodoStore::default();
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
        .route(
            "/todos",
            get(todos_handler)
                .post(create_todos_handler)
                .layer(Extension(store.clone())), // 为 /todos 路径添加状态共享
        )
        .route("/login", post(login_handler))
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

async fn todos_handler(
    Extension(store): Extension<TodoStore>,
    claims: Claims,
) -> Result<Json<Vec<Todo>>, HttpError> {
    let user_id = claims.id;
    match store.todos.read() {
        Ok(todos) => {
            let todos: Vec<Todo> = todos
                .iter()
                .filter(|todo| todo.user_id == user_id)
                .cloned()
                .collect();
            Ok(Json(todos))
        }
        Err(_) => Err(HttpError::Internal),
    }
}

async fn create_todos_handler(
    Extension(store): Extension<TodoStore>,
    claims: Claims,
    Json(todo): Json<CreateTodo>,
) -> Result<StatusCode, HttpError> {
    match store.todos.write() {
        Ok(mut todos) => {
            let new_todo = Todo {
                id: get_next_id(),
                user_id: claims.id,
                title: todo.title,
                completed: false,
            };
            todos.push(new_todo);
            Ok(StatusCode::CREATED)
        }
        Err(_) => return Err(HttpError::Internal),
    }
}
//eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6ImEifQ.dI5wbU5qoN7gqsHnPVDVhNA95UjlR--__yw6gw8AKKc
async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    let claims = Claims {
        id: 1,
        name: login.email,
    };
    let token = jwt::encode(
        &jwt::Header::default(),
        &claims,
        &jwt::EncodingKey::from_secret(SECRET_KEY),
    )
    .unwrap();
    Json(LoginResponse { token })
}

fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
