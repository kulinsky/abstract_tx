use async_trait::async_trait;

use crate::{err::Error, tx_manager::TransactionManager};

#[async_trait]
pub trait Repository {
    type TxMgr: TransactionManager;

    async fn save<'a>(
        &self,
        name: &str,
        tx: &mut <Self::TxMgr as TransactionManager>::Tx<'a>,
    ) -> Result<(), Error>;
}

#[async_trait]
pub trait SecondRepository {
    type TxMgr: TransactionManager;

    async fn save<'a>(
        &self,
        name: &str,
        tx: &mut <Self::TxMgr as TransactionManager>::Tx<'a>,
    ) -> Result<(), Error>;
}
