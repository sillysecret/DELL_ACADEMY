
use crate::{User, UserDTS, Aposta,ApostaDTS,Mega, MegaDTS};
use sqlx::{postgres::PgPoolOptions, PgPool};
use time::OffsetDateTime;
use uuid::Uuid;




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
            RETURNING ID, NOME, CPF
            ",
            newid,
            user.cpf,
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
                INSERT INTO MEGA (ID, DATA_,FK_USER_ID)
                VALUES ($1, $2, $3)
                RETURNING ID, DATA_, FK_USER_ID
                ",
                newid,
                formatted_time,
                mega.user_id
            )
            .fetch_one(&self.pool)
            .await
        }else{
            Err(sqlx::error::Error::WorkerCrashed)
        }
     
    }

    pub async fn create_adm(&self , user:UserDTS) -> Result<User, sqlx::Error>{
        let newid = Uuid::now_v7();
        let auth = 1;
        
        sqlx::query_as!(
            User,
            "
            INSERT INTO USUARIO (ID, CPF, NOME, FK_AUTH_ID)
            VALUES ($1, $2, $3, $4)
            RETURNING ID, NOME, CPF
            ",
            newid,
            user.cpf,
            user.nome,
            auth
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn matchresult(&self , randvec:Vec<i32>, id:Uuid) -> Result<Vec<Aposta>, sqlx::Error>{
        
        
        sqlx::query_as!(
            Aposta,
            "
            SELECT *
            FROM APOSTA
            WHERE FK_MEGA_ID = $1
            AND ARRAY(SELECT unnest($2::integer[])) <@ ANY(SELECT ARRAY_AGG(VEC) FROM APOSTA WHERE FK_MEGA_ID = $1)
            ",
            id,
            &randvec
        ).fetch_all(&self.pool).await
        
        
        
    }

}
