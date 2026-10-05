/// A failure while preparing or validating an existing tool lockfile.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ToolLockError {
    #[error(transparent)]
    Validation(#[from] uv_lock_operations::LockValidationError),
    #[error(transparent)]
    ClientBuild(#[from] uv_client::ClientBuildError),
    #[error(transparent)]
    FlatIndex(Box<uv_client::FlatIndexError>),
    #[error(transparent)]
    HashStrategy(#[from] uv_types::HashStrategyError),
}

impl From<uv_client::FlatIndexError> for ToolLockError {
    fn from(error: uv_client::FlatIndexError) -> Self {
        Self::FlatIndex(Box::new(error))
    }
}
