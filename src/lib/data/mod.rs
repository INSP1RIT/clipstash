pub mod model;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};
use std::str::FromStr;
use sqlx::{Sqlite, SqlitePool};
use sqlx;
use sqlx::sqlite::{SqliteQueryResult, SqliteRow};

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    DataBase(#[from] sqlx::Error)
}

pub type AppDatabase = Database<Sqlite>;
pub type DataBasePool = SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;

pub type AppDatabaseRaw = SqliteRow;
pub type AppQueryResult = SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new().
            connect(connection_str).await;

        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!(r"If the database has not been created yet, run \n $sqlx database setup \n");
                panic!("Database connection error!")
            }
        }
    }

    pub fn get_pool(&self) -> &DataBasePool {
        &self.0
    }
}

#[derive(From, Clone, Debug, Display, Serialize, Deserialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(s)?))
    }
}