use async_trait::async_trait;

use crate::err::Error;

#[async_trait]
pub trait TransactionManager {
    type Tx<'a>;

    async fn begin<'a>(&self) -> Result<Self::Tx<'a>, Error>;

    async fn commit<'a>(&self, tx: Self::Tx<'a>) -> Result<(), Error>;

    async fn rollback<'a>(&self, tx: Self::Tx<'a>) -> Result<(), Error>;
}
