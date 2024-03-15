use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,extract::{State,Path}, Json, http::StatusCode,
};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use time::Date;
use std::env;
use std::sync::Arc;
use database::Repository;



mod database;

type AppState = Arc<Repository>;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Deserialize)]
pub struct Querysearch{
    pub query: String
}


#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct User{
    pub id: Uuid,
    pub cpf: String,
    pub nome: String,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct UserDTS{
    pub nome: String,
    pub cpf: String,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct Mega{
    pub id: Uuid,
    pub data_: Date,
    pub fk_user_id: Uuid,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct MegaDTS{
    pub aval: bool,
    pub user_id: Uuid,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct Aposta{
    pub id: i32,
    pub fk_user_id: Uuid,
    pub fk_mega_id: Uuid,
    pub vec: Vec<i32>,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct ApostaDTS{
    pub fk_user_id: Uuid,
    pub fk_mega_id: Uuid,
    pub vec: Vec<i32>,
}

#[tokio::main]
async fn main() {
    
    // build our application with a single route
    
    let port =env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://dell:dell@localhost:5432/dell"));
   
    let db = Repository::conn(port).await;

    let app_state = Arc::new(db);
 
    let app = Router::new()
        .route("/user",post(make_user))
        .route("/aposta",post(make_aposta))
        .route("/mega", post(make_mega))
        .route("/useradm", post(make_adm))
        .route("/startmega", get(start_mega))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn make_user(State(localbd): State<AppState>,Json(payload): Json<UserDTS>)-> impl IntoResponse{        
    match localbd.create_user(payload).await{
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar usuario"))) ,
        
    }
}

async fn make_adm(State(localbd): State<AppState>,Json(payload): Json<UserDTS>) -> impl IntoResponse {
    match localbd.create_adm(payload).await{
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar adm"))) 
    }

    // dps tratar com cors
}

async fn make_mega(State(localbd): State<AppState>,Json(payload): Json<MegaDTS>) -> impl IntoResponse {
    match localbd.create_mega(payload).await{
        Ok(mega) => Ok((StatusCode::CREATED, Json(mega))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar mega"))) 
    }
    
}

async fn make_aposta(State(localbd): State<AppState>,Json(payload): Json<ApostaDTS>) -> impl IntoResponse {
    match localbd.create_aposta(payload).await{
        Ok(aposta) => Ok((StatusCode::CREATED, Json(aposta))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar aposta"))) 
    }
} 

async fn start_mega(State(localbd): State<AppState>,Path(id): Path<Uuid>,) -> impl IntoResponse {
    todo!()
}





