use crate::{
    err::Error,
    repository::{Repository, SecondRepository},
    tx_manager::TransactionManager,
};

pub struct App<TxManager, R, S>
where
    TxManager: TransactionManager,
    R: Repository,
    S: SecondRepository,
{
    repository: R,
    second_repository: S,
    tx_manager: Box<TxManager>,
}

impl<TxManager, R, S> App<TxManager, R, S>
where
    TxManager: TransactionManager,
    R: Repository<TxMgr = TxManager>,
    S: SecondRepository<TxMgr = TxManager>,
{
    pub fn new(repository: R, second_repository: S, tx_manager: Box<TxManager>) -> Self {
        Self {
            repository,
            second_repository,
            tx_manager,
        }
    }

    pub async fn save(&self) -> Result<(), Error> {
        let mut tx = self.tx_manager.begin().await?;

        self.repository.save("from main", &mut tx).await?;

        self.second_repository.save("from second", &mut tx).await?;

        self.tx_manager.commit(tx).await?;

        Ok(())
    }
}
