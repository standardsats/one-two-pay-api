use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Error {
    #[error("")]
    Dummy(),
}
