use async_trait::async_trait;

use crate::err::Error;
use crate::repository::{Repository, SecondRepository};
use crate::tx_manager::TransactionManager;

pub struct InMemoryTxManager;

#[async_trait]
impl TransactionManager for InMemoryTxManager {
    type Tx<'a> = String;

    async fn begin<'a>(&self) -> Result<Self::Tx<'a>, Error> {
        Ok("in_memory_tx".to_string())
    }

    async fn commit<'a>(&self, tx: Self::Tx<'a>) -> Result<(), Error> {
        println!("commit: {}", tx);

        Ok(())
    }

    async fn rollback<'a>(&self, tx: Self::Tx<'a>) -> Result<(), Error> {
        println!("rollback: {}", tx);

        Ok(())
    }
}

pub struct InMemoryRepository;

#[async_trait]
impl Repository for InMemoryRepository {
    type TxMgr = InMemoryTxManager;

    async fn save<'a>(
        &self,
        name: &str,
        _tx: &mut <Self::TxMgr as TransactionManager>::Tx<'a>,
    ) -> Result<(), Error> {
        println!("save: {}", name);

        Ok(())
    }
}

pub struct InMemorySecondRepository;

#[async_trait]
impl SecondRepository for InMemorySecondRepository {
    type TxMgr = InMemoryTxManager;
    async fn save<'a>(
        &self,
        name: &str,
        _tx: &mut <Self::TxMgr as TransactionManager>::Tx<'a>,
    ) -> Result<(), Error> {
        println!("save: {}", name);

        Ok(())
    }
}
