use async_trait::async_trait;
use dotenv::dotenv;
use sqlx::{
    migrate, migrate::MigrateError, query, query_as, sqlite::SqlitePoolOptions, Pool, Sqlite,
};
use std::env;
use thiserror::Error;

#[derive(Debug)]
pub struct MyDB {
    pool: Pool<Sqlite>,
}

#[async_trait]
impl CRUD for MyDB {
    async fn new() -> Result<MyDB, DbError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE URL NOT FOUND");
        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        migrate!("./migrations").run(&db_pool).await?;
        Ok(MyDB { pool: db_pool })
    }
    async fn create_user(&self, username: &str, email: &str) -> Result<i64, DbError> {
        let res = query!(
            "INSERT INTO users (username, email) VALUES ($1, $2);",
            username,
            email,
        )
        .execute(&self.pool)
        .await?;
        Ok(res.last_insert_rowid())
    }
    async fn update_user(
        &self,
        id: i64,
        username: Option<&str>,
        email: Option<&str>,
    ) -> Result<(), DbError> {
        if let Some(username) = username {
            query!("UPDATE users SET username=$2 WHERE id=$1", id, username)
                .execute(&self.pool)
                .await?;
            return Ok(());
        };
        if let Some(email) = email {
            query!("UPDATE users SET email=$2 WHERE id=$1", id, email)
                .execute(&self.pool)
                .await?;
            return Ok(());
        };
        Err(MyError::NoFieldsSet)?
    }

    async fn find_user(&self, id: Option<i64>, email: Option<&str>) -> Result<User, DbError> {
        if let Some(id) = id {
            let user = query_as!(User, "SELECT * FROM users WHERE id=$1", id)
                .fetch_one(&self.pool)
                .await?;
            return Ok(user);
        }
        if let Some(email) = email {
            let user = query_as!(User, "SELECT * FROM users WHERE email=$1", email)
                .fetch_one(&self.pool)
                .await?;
            return Ok(user);
        }
        Err(MyError::NoFieldsSet)?
    }

    async fn delete_user(&self, id: i64) -> Result<(), DbError> {
        query!("DELETE FROM users WHERE id=$1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub author: i64,
    pub published: bool,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    EnvError(#[from] env::VarError),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    ManError(#[from] MyError),
    #[error(transparent)]
    MigrateError(#[from] MigrateError),
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("this is not implemented yet")]
    Unimplemented,
    #[error("not enough fields supplid for method")]
    NoFieldsSet,
}

#[async_trait]
pub trait CRUD {
    async fn new() -> Result<MyDB, DbError>;
    async fn create_user(&self, username: &str, email: &str) -> Result<i64, DbError>;
    async fn update_user(
        &self,
        id: i64,
        username: Option<&str>,
        email: Option<&str>,
    ) -> Result<(), DbError>;
    async fn find_user(&self, id: Option<i64>, email: Option<&str>) -> Result<User, DbError>;
    async fn delete_user(&self, id: i64) -> Result<(), DbError>;
}
