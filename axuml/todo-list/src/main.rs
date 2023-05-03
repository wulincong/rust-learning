use std::net::SocketAddr;
use std::sync::RwLock;
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader, Extension},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router, Server, Json, headers::{Authorization, authorization::Bearer}, 
    AddExtensionLayer,
    
};
use jsonwebtoken as jwt;
use jsonwebtoken::Validation;

const SECRET_KEY: &[u8] = b"deadbeef";

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo{
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo{
    pub title:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest{
    email: String,
    password: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse{
    token: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    id: usize,
    name: String,
    exp: usize,
}

#[derive(Debug, Default, Clone)]
struct TodoStore{
    items: Arc<RwLock<Vec<Todo>>>,
}


#[tokio::main]
async fn main() {
    let store = TodoStore::default();
    let app = Router::new()
        .route("/", get(index_handler))
        .route(
            "/todos", 
            get(todos_handler)
            .post(create_todos_handler)
            .layer(AddExtensionLayer::new(store)))
        .route("/login", post(login_handler));

    let addr = SocketAddr::from(([127,0,0,1], 8848));
    println!("Listening in http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn index_handler() -> Html<&'static str>{
    Html("hello world")
}

async fn todos_handler() -> Json<Vec<Todo>>{
    Json(vec![
        Todo{
            id: 1,
            title: "Todo 1".to_string(),
            completed: false,
        },
        Todo{
            id: 2,
            title: "Todo 2".to_string(),
            completed: false,
        },
    ])
}

//eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkpvaG4gRG93IiwiZXhwIjoxNjg0MzE2MjA4fQ.fdopK3tKSZwbQt9DpvtfuVNnKXy71sXgeDWK4osQJmI

async fn create_todos_handler(
    claims:Claims, 
    Json(_todo):Json<CreateTodo>, 
    Extension(todos):Extension<TodoStore>) 
-> Result<StatusCode, HttpError>{
    match store.items.write(){
        Ok(guard) =>{
            let todo = Todo{
                id: get_next_id(),
                title: todo.title,
                completed: false,
            };
            guard.push(todo);
            Ok(StatusCode::CREATED)
        }
        Err(_) => Err(HttpError::Internal)
    }
    println!("{:?}",claims);
    StatusCode::CREATED
}

async fn login_handler(Json(_login):Json<LoginRequest>) -> Json<LoginResponse>{
    let claims = Claims{
        id:1,
        name: "John Dow".to_string(),
        exp: get_epoch() + 14 * 24 * 60 * 60,
    };
    let key = jwt::EncodingKey::from_secret(SECRET_KEY);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse{token})
}


#[async_trait]
impl<B> FromRequest<B> for Claims
where 
    B:Send{
        type Rejection = HttpError;
        async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection>{
            let TypedHeader(Authorization(bearer)) = 
                TypedHeader::<Authorization<Bearer>>::from_request(req)
                    .await
                    .map_err(|_| HttpError::Auth)?;
            let key = jwt::DecodingKey::from_secret(SECRET_KEY);
            let token = jwt::decode::<Claims>(bearer.token(), &key, &Validation::default()).map_err(|e | {
                println!("{:?}",e);
                HttpError::Auth
            })?;
            Ok(token.claims)
        }
    }


#[derive(Debug)]

pub enum HttpError{
    Auth,
    Internal,
}


impl IntoResponse for HttpError{
    fn into_response(self) -> axum::response::Response{
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        (code, msg).into_response()
    }
}

fn get_epoch() -> usize{
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}
