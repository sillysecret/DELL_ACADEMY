
use crate::{Aposta, ApostaDTS, Frequencia, Mega, MegaDTS, User, UserDTS};
use sqlx::{postgres::PgPoolOptions, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;


fn remover_nao_digitos(texto: &str) -> String {
    texto.chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

pub struct Repository{
    pool: PgPool, 
}


impl Repository {
    pub async fn conn(url : String) -> Self {
        Repository{
        pool : PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap(),
        }
    }

    pub async fn create_user(&self , user:UserDTS) -> Result<User, sqlx::Error>{
        let newid = Uuid::now_v7();
        let auth = 0;
        
        sqlx::query_as!(
            User,
            "
            INSERT INTO USUARIO (ID, CPF, NOME, FK_AUTH_ID)
            VALUES ($1, $2, $3, $4)
            RETURNING ID, NOME, CPF, FK_AUTH_ID
            ",
            newid,
            remover_nao_digitos(&user.cpf),
            user.nome,
            auth
        )
        .fetch_one(&self.pool)
        .await
        
       
    }

    pub async fn create_aposta(&self , newaposta:ApostaDTS) -> Result<Aposta, sqlx::Error>{    


        sqlx::query_as!(
            Aposta,
            "
            INSERT INTO APOSTA (FK_USER_ID, FK_MEGA_ID, VEC)
            VALUES ($1, $2, $3)
            RETURNING ID, FK_USER_ID, FK_MEGA_ID, VEC
            ",
            newaposta.fk_user_id,
            newaposta.fk_mega_id,
            &newaposta.vec
        ).fetch_one(&self.pool)
        .await


    }

    pub async fn create_mega(&self , mega:MegaDTS) -> Result<Mega, sqlx::Error>{
        let newid = Uuid::now_v7();
        let current_time = OffsetDateTime::now_utc();
        let formatted_time = current_time.date();

        if mega.aval{
            sqlx::query_as!(
                Mega,
                "
                INSERT INTO MEGA (ID, DATA_,AMOUNT,FK_USER_ID)
                VALUES ($1, $2, $3, $4)
                RETURNING ID, DATA_,AMOUNT, FK_USER_ID
                ",
                newid,
                formatted_time,
                mega.amount,
                mega.user_id
            )
            .fetch_one(&self.pool)
            .await
        }else{
            Err(sqlx::error::Error::WorkerCrashed)
        }
     
    }

    // pub async fn create_adm(&self , user:UserDTS) -> Result<User, sqlx::Error>{
    //     let newid = Uuid::now_v7();
    //     let auth = 1;
        
    //     sqlx::query_as!(
    //         User,
    //         "
    //         INSERT INTO USUARIO (ID, CPF, NOME, FK_AUTH_ID)
    //         VALUES ($1, $2, $3, $4)
    //         RETURNING ID, NOME, CPF
    //         ",
    //         newid,
    //         user.cpf,
    //         user.nome,
    //         auth
    //     )
    //     .fetch_one(&self.pool)
    //     .await
    // }

    pub async fn matchresult(&self , randvec:Vec<i32>, id:Uuid) -> Result<Vec<Aposta>, sqlx::Error>{
        
        
        sqlx::query_as!(
            Aposta,
            "
                SELECT *
                FROM APOSTA
                WHERE FK_MEGA_ID = $1
                GROUP BY id
                HAVING array_contains_all(
                    ARRAY_AGG(VEC),
                    ARRAY(SELECT unnest($2::integer[]))
                )
            ",
            id,
            &randvec
        ).fetch_all(&self.pool).await
        
        
        
    }

    pub async fn login_user(&self , cpf:String) -> Result<User, sqlx::Error>{
        sqlx::query_as!(
            User,
            "
            SELECT ID, NOME, CPF,FK_AUTH_ID
            FROM USUARIO
            WHERE CPF = $1
            ",
            cpf
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_frequent_numbers(&self , id:Uuid) -> Result<Vec<Frequencia>, sqlx::Error>{
        sqlx::query_as!(
            Frequencia,
            "
            SELECT CAST(numero AS INTEGER) AS numero, COUNT(numero) AS frequencia
            FROM (
                SELECT UNNEST(VEC) AS numero
                FROM APOSTA 
                WHERE FK_MEGA_ID = $1
            ) AS numeros_desagregados
            GROUP BY numero
            ORDER BY frequencia DESC
            LIMIT 5;                
            ",
            id
        ).fetch_all(&self.pool)
        .await

    } 

    pub async fn get_mega(&self ) -> Result<Vec<Mega>, sqlx::Error>{
        sqlx::query_as!(
            Mega,
            "
            SELECT *
            FROM MEGA 
            "
        ).fetch_all(&self.pool)
        .await
    }

    pub async fn get_recenct_mega(&self ) -> Result<Mega, sqlx::Error>{
        sqlx::query_as!(
            Mega,
            "
            SELECT *   
            FROM MEGA 
            ORDER BY DATA_ DESC
            LIMIT 1
            "
        ).fetch_one(&self.pool)
        .await

    }


}
