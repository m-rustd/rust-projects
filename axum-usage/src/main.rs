use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use axum::{
    async_trait,
    extract::{FromRequestParts, Query, Path},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode, header, Uri},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router, Server, TypedHeader, body::{Full, boxed},
};
use axum_usage::{HttpError, CommonClaim, ws_handler, ChatState};
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

#[derive(Debug, Default, Deserialize)]
struct UserCreate {
    name: String,
}

#[derive(RustEmbed)]
#[folder = "example/public/"]
struct Assets;

struct StaticFile<T>(T);

impl <T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        match Assets::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mine = mime_guess::from_path(path.as_str()).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mine.as_ref())
                    .body(body)
                    .unwrap()
            },
            None => {
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(boxed(Full::from(format!("File {} not found", path))))
                    .unwrap()
            },
        }
    }
    
}


async fn hello_handler() -> impl IntoResponse {
    // StaticFile("index.html")
    static_handler("/index.html".parse().unwrap()).await
}

async fn static_handler(url: Uri) -> impl IntoResponse {
    let path = url.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

async fn hi_handler(query: Query<Todo>) -> Json<String> {
    println!("query: {:?}", query);
    Json("<h1>Hello, World!</h1>".into())
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
    claims: CommonClaim<Claims>,
) -> Result<Json<Vec<Todo>>, HttpError> {
    let user_id = claims.0.id;
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

async fn users_show(id: Path<usize>, query: Option<Query<UserCreate>>) -> Json<String> {
    let Query(query) = query.unwrap_or_default();
    if query.name.is_empty() {
        return Json("name is empty".into());
    }
    Json(format!("user id: {:?} name: {}", id, query.name))
}

// 获取系统时间
fn get_epoch() -> usize {
    let now = std::time::SystemTime::now();
    now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as usize
}

fn get_next_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let store = TodoStore::default();

    let app = Router::new()
        .route("/", get(|| async { "Hello, index!" }))
        .route("/hi", get(hi_handler))
        .route("/hello", get(hello_handler))
        .route("/login", post(login_handler))
        .route(
            "/todos",
            get(todos_handler)
            .post(create_todos_handler)
            .layer(Extension(store)),
        )
        .route("/users/:id", get(users_show))
        .route("/ws", get(ws_handler).layer(Extension(ChatState::default())))
        .fallback(get(static_handler))
        ;
    println!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
