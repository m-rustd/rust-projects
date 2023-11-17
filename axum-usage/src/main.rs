use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router, Server, TypedHeader,
};
use jsonwebtoken as jwt;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};

// secret key
const SECRET_KEY: &[u8] = b"deadbeer";
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       
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
    exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| HttpError::UnAuthorized)?;
        let key = jwt::DecodingKey::from_secret(SECRET_KEY);
        let token = jwt::decode::<Claims>(bearer.token(), &key, &jwt::Validation::default())
            .map_err(|_| HttpError::UnAuthorized)?;

        Ok(token.claims)
    }
}

#[derive(Debug)]
enum HttpError {
    UnAuthorized,
    InternalServerError,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::UnAuthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            HttpError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };
        (code, msg).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    user_id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct CrateTodo {
    title: String,
}

#[derive(Debug, Default, Clone)]
struct TodoStore {
    items: Arc<RwLock<Vec<Todo>>>,
}

#[derive(RustEmbed)]
#[folder = "example/public/"]
struct Assets;

// impl Assets {
//     pub fn get(file_path: &str) -> Option<rust_embed::EmbeddedFile> {
//         println!("file_path: {}", file_path);
//     }
// }

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let store = TodoStore::default();

    let app = Router::new()
        .route("/", get(|| async { "Hello, index!" }))
        .route("/hello", get(hello_handler))
        .route("/login", post(login_handler))
        .route(
            "/todos",
            get(todos_handler)
                .post(create_todos_handler)
                .layer(Extension(store)),
        );
    println!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> Html<String> {
    Html("<h1>Hello, World!</h1>".into())
}

// post login json
// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IjExMUBxcS5jb20iLCJleHAiOjE3MDE0MTI3NDJ9.5tlzJmB_3c4w687hT0J6Oc7y-8iCeSY16UXdaGxz78w                                                                                                                                                                                                                                                                                                                                                                                 eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IjExMUBxcS5jb20ifQ.Se1kdqua3O0qtZEMI-UIKzGwHsMUlxcbumzjdVqE3R8
async fn login_handler(login: Json<LoginRequest>) -> Json<LoginResponse> {
    let claims = Claims {
        id: 1,
        name: login.email.clone(),
        exp: get_epoch() + 14 * 24 * 60 * 60,
    };
    let key = jwt::EncodingKey::from_secret(SECRET_KEY);
    let token: String = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

// get /todos list
async fn todos_handler(
    store: Extension<TodoStore>,
    claims: Claims,
) -> Result<Json<Vec<Todo>>, HttpError> {
    let user_id = claims.id;
    match store.items.read() {
        Ok(items) => Ok(Json(
            items
                .iter()
                .filter(|todo| todo.user_id == user_id)
                .cloned()
                .collect(),
        )),
        Err(_) => Err(HttpError::InternalServerError),
    }
}

// post /todos json
async fn create_todos_handler(
    store: Extension<TodoStore>,
    claims: Claims,
    create_todo: Json<CrateTodo>,
) -> Result<StatusCode, HttpError> {
    println!("claims: {:?}", claims);
    match store.items.write() {
        Ok(mut guard) => {
            let todo = Todo {
                id: get_next_id(),
                user_id: claims.id,
                title: create_todo.title.clone(),
                completed: false,
            };
            guard.push(todo);
            Ok(StatusCode::CREATED)
        }
        Err(_) => Err(HttpError::InternalServerError),
    }
}

// 获取系统时间
fn get_epoch() -> usize {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize
}

fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
