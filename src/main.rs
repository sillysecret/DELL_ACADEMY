use axum::{
    extract::{rejection::FailedToDeserializeForm, Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use time::Date;
use std::{collections::hash_map::Iter, env, f32::consts::E, vec};
use std::sync::Arc;
use database::Repository;
use rand::Rng;



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

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow,Debug)]
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
        .route("/startmega/:id", get(start_mega))
        .with_state(app_state);

        //FAZER UM LAYER DE CORS

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn make_user(State(localbd): State<AppState>,Json(payload): Json<UserDTS>)-> impl IntoResponse{        
    if payload.nome.len() > 100{
        return Err((StatusCode::BAD_REQUEST, Json("Nome muito grande")));
    }
    
    if payload.cpf.len() > 20{
        return Err((StatusCode::BAD_REQUEST, Json("CPF muito grande")));
    }
    
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
    
    // verifica o tamanho do vetor 

    if payload.vec.len() != 5 {
        return Err((StatusCode::BAD_REQUEST, Json("A aposta deve conter 5 numeros")));
    }

    // verifica a faixa dos numeros
    if payload.vec.iter().any(|&x| x > 50 || x < 1) {
        return Err((StatusCode::BAD_REQUEST, Json("Os numeros devem estar entre 1 e 50")));
    }
   
    match localbd.create_aposta(payload).await{
        Ok(aposta) => Ok((StatusCode::CREATED, Json(aposta))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar aposta"))) 
    }
} 

async fn start_mega(State(localbd): State<AppState>,Path(id): Path<Uuid>) -> impl IntoResponse {
     let mut vec = Vec::new();
     for _ in 0..5{
          vec.push(rand::thread_rng().gen_range(1..50));
     }
     let mut retries = 0;    
     const MAX_RETRIES: u32 = 25;
     let mut vec_clone = vec.clone();
     
     loop{
        print!("Tentativa: {}", retries);
        println!("vec_clone: {:?}", vec_clone);

        match localbd.matchresult(vec_clone.clone(), id).await {
            Ok(apostas) => {               
                if apostas.is_empty(){
                    vec_clone.push(rand::thread_rng().gen_range(1..50));
                    retries += 1;

                    println!("apostas : {:?}", apostas);

                    if retries >= MAX_RETRIES {
                        println!("Número máximo de tentativas atingido!");
                        break;
                    }
                }
                else{
                    return Ok((StatusCode::OK, Json(apostas)));   
                }
                
            }
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar aposta vencedora")));
                break;
            }
        }
    }

    Err((StatusCode::NOT_FOUND, Json("nenhuma aposta encontrada")))

}












