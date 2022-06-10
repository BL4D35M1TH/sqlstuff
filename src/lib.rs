use async_trait::async_trait;
use dotenv::dotenv;
use sqlx::{
    migrate, migrate::MigrateError, query_as, sqlite::SqlitePoolOptions, Decode, Encode, Pool,
    Sqlite,
};
use std::env;

#[derive(Debug)]
pub struct MyDB {
    pool: Pool<Sqlite>,
}

#[async_trait]
impl CRUD for MyDB {
    async fn create_user(&self, username: &str, email: &str) -> Result<User, DbError> {
        query_as!(
            User,
            "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *;",
            username,
            email
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|err| err.into())
    }

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
}

#[derive(Encode, Decode, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Encode, Decode, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub author: i64,
    pub published: bool,
}

#[derive(Debug)]
pub enum DbError {
    VarError(env::VarError),
    SqlxError(sqlx::Error),
    StrError(String),
    MigrateError(MigrateError),
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        DbError::SqlxError(err)
    }
}
impl From<env::VarError> for DbError {
    fn from(err: env::VarError) -> Self {
        DbError::VarError(err)
    }
}
impl From<String> for DbError {
    fn from(err: String) -> Self {
        DbError::StrError(err)
    }
}
impl From<MigrateError> for DbError {
    fn from(err: MigrateError) -> Self {
        DbError::MigrateError(err)
    }
}

#[async_trait]
pub trait CRUD {
    async fn create_user(&self, username: &str, email: &str) -> Result<User, DbError>;
    async fn new() -> Result<MyDB, DbError>;
}
