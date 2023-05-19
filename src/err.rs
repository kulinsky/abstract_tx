use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Repository Error: {0}")]
    Repository(String),
}
