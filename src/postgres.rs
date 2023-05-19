use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    err::Error,
    repository::{Repository, SecondRepository},
    tx_manager::TransactionManager,
};

pub struct SQLXTransactionManager {
    pool: PgPool,
}

impl SQLXTransactionManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionManager for SQLXTransactionManager {
    type Tx<'a> = Transaction<'a, Postgres>;

    async fn begin<'a>(&self) -> Result<Self::Tx<'a>, Error> {
        let tx = self.pool.begin().await?;

        Ok(tx)
    }

    async fn commit<'a>(&self, tx: Self::Tx<'a>) -> Result<(), Error> {
        tx.commit().await?;

        Ok(())
    }

    async fn rollback<'a>(&self, tx: Self::Tx<'a>) -> Result<(), Error> {
        tx.rollback().await?;

        Ok(())
    }
}

pub struct SQLXRepository {
    pool: PgPool,
}

impl SQLXRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::Repository(err.to_string())
    }
}

#[async_trait]
impl Repository for SQLXRepository {
    type TxMgr = SQLXTransactionManager;

    async fn save<'a>(
        &self,
        name: &str,
        tx: &mut <Self::TxMgr as TransactionManager>::Tx<'a>,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO users (name) VALUES ($1)")
            .bind(name)
            .execute(tx)
            .await?;
        Ok(())
    }
}

pub struct SQLXSecondRepository {
    pool: PgPool,
}

impl SQLXSecondRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SecondRepository for SQLXSecondRepository {
    type TxMgr = SQLXTransactionManager;

    async fn save<'a>(
        &self,
        name: &str,
        tx: &mut <Self::TxMgr as TransactionManager>::Tx<'a>,
    ) -> Result<(), Error> {
        sqlx::query("INSERT INTO users (name) VALUES ($1)")
            .bind(name)
            .execute(tx)
            .await?;
        Ok(())
    }
}
