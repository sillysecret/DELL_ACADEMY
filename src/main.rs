use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Serialize,Deserialize};
use uuid::Uuid;
use time::Date;
use std::env;
use std::sync::Arc;
use database::Repository;
use rand::Rng;
use tower_http::cors::{CorsLayer,Any};


//fazer endpont de consultar todas as apostas
//adicionar sistema de senhas 
//fazer tela de adm
//adicionar campo de validacao mega(para ver se a mega esta ativa ou acabou)

mod database;

type AppState = Arc<Repository>;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Deserialize)]
pub struct Querysearch{
    pub query: String
}
#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct Count{
    pub count: Option<i64>,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct User{
    pub id: Uuid,
    pub cpf: String,
    pub nome: String,
    pub wallet: f64,
    pub fk_auth_id: i32,
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
    pub amount: i32,
    pub avaliable: bool,
    pub fk_user_id: Uuid,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct MegaDTS{
    pub aval: bool,
    pub user_id: Uuid,
    pub amount: i32,
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

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct Frequencia {
    pub numero: Option<i32>,
    pub frequencia:Option<i64>,
}

#[derive(Serialize,Clone,Deserialize,sqlx::FromRow)]
pub struct Apostaview{
    pub id: i32,
    pub fk_mega_id: Uuid,
    pub vec: Vec<i32>,
    pub user_username: String,
}




#[tokio::main]
async fn main() {
    
    // build our application with a single route
    
    let port =env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://dell:dell@localhost:5432/dell"));
   
    let db = Repository::conn(port).await;

    let app_state = Arc::new(db);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_private_network(true);
 
    let app = Router::new()
        .route("/user",post(make_user))
        .route("/aposta",post(make_aposta))
        .route("/mega", post(make_mega))
        .route("/startmega/:id", get(start_mega))
        .route("/loginuser", post(loginuser))
        .route("/analize/:id", get(get_nums_of_mega))
        .route("/megas", get(get_megas))
        .route("/getrecent", get(get_recenct_mega))
        .route("/getallapostas/:id", get(get_all_apostas))
        .layer(cors)
        .with_state(app_state);
        

        //FAZER UM LAYER DE CORS

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



async fn loginuser(State(localbd): State<AppState>,Json(payload): Json<UserDTS>) -> impl IntoResponse {
    //arrumar cpf e colocar como unique
    match localbd.login_user(payload.cpf).await{
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("user não encontrado"))) 
    }
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
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() =>Err((StatusCode::UNPROCESSABLE_ENTITY,Json("Unnique violation"))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar usuario"))) ,
        
    }
}

async fn make_mega(State(localbd): State<AppState>,Json(payload): Json<MegaDTS>) -> impl IntoResponse {
    
    match localbd.get_mega().await{
        Ok(count) => {
            if let Some(value) = count.count {
                if value >= 1 {
                    return Err((StatusCode::BAD_REQUEST, Json("Mega ja criada")));
                }
            } 
        }
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar megas")))

    }
    
    match localbd.create_mega(payload).await{
        Ok(mega) => Ok((StatusCode::CREATED, Json(mega))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao criar mega"))) 
    }
    
}

async fn make_aposta(State(localbd): State<AppState>,Json(payload): Json<ApostaDTS>) -> impl IntoResponse {
    // verificar se a aposta ja foi feita com os memsmos numeros    

    if payload.vec.len() != 5 {
        return Err((StatusCode::BAD_REQUEST, Json("A aposta deve conter 5 numeros")));
    }

    // verifica a faixa dos numeros
    if payload.vec.iter().any(|&x| x > 50 || x < 1) {
        return Err((StatusCode::BAD_REQUEST, Json("Os numeros devem estar entre 1 e 50")));
    }
    
    //verificar se o user mandou numeros repetidos no vec
    if payload.vec.iter().any(|&x| payload.vec.iter().filter(|&y| *y == x).count() > 1){
        return Err((StatusCode::BAD_REQUEST, Json("Os numeros devem ser diferentes")));
    }

   
    match localbd.create_aposta(payload).await{
        Ok(aposta) => Ok((StatusCode::CREATED, Json(aposta))),
        Err(sqlx::Error::WorkerCrashed) =>Err((StatusCode::UNPROCESSABLE_ENTITY,Json("aposta ja feita com esses numero"))),
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
                    add_randown_number_not_contained(&mut vec_clone);
                    retries += 1;
                    println!("apostas : {:?}", apostas);
                    if retries >= MAX_RETRIES {
                        println!("Número máximo de tentativas atingido!");
                        break;
                    }
                }
                else{
                    //let _ =localbd.disable_mega(id).await;
                    return Ok((StatusCode::OK, Json(apostas)));   
                }
            }
            Err(_e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar aposta vencedora")));
            }
        }
    }

    Err((StatusCode::OK, Json("nenhuma aposta encontrada")))

}

async fn get_nums_of_mega(State(localbd): State<AppState>,Path(id): Path<Uuid>)-> impl IntoResponse{
    match localbd.get_frequent_numbers(id).await{
        Ok(frequencia) => Ok((StatusCode::OK, Json(frequencia))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar frequencia"))) 
    }
}

async fn get_megas(State(localbd): State<AppState>)-> impl IntoResponse{
    match localbd.get_mega().await{
        Ok(megas) => Ok((StatusCode::OK, Json(megas))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar megas"))) 
    }
}

async fn get_recenct_mega(State(localbd): State<AppState>)-> impl IntoResponse{
    match localbd.get_recenct_mega().await{
        Ok(mega) => Ok((StatusCode::OK, Json(mega))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar mega recente"))) 
    }
}

async fn get_all_apostas(State(localbd): State<AppState>,Path(id): Path<Uuid>)-> impl IntoResponse{
    match localbd.get_all_apostas(id).await{
        Ok(apostas) => Ok((StatusCode::OK, Json(apostas))),
        Err(_) =>Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Erro ao buscar apostas"))) 
    }
}

fn add_randown_number_not_contained(vec: &mut Vec<i32>){
    let mut rng = rand::thread_rng();
    let mut number = rng.gen_range(1..50);
    while vec.contains(&number){
        number = rng.gen_range(1..50);
    }
    vec.push(number);
}










