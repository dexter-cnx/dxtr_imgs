use std::path::PathBuf;

use thiserror::Error;

pub trait PlatformFileDialog: Send + Sync {
    fn pick_files(&self) -> Result<Vec<PathBuf>, PlatformError>;
    fn pick_folder(&self) -> Result<Option<PathBuf>, PlatformError>;
}

pub trait PlatformPaths: Send + Sync {
    fn app_support_dir(&self) -> Result<PathBuf, PlatformError>;
    fn cache_dir(&self) -> Result<PathBuf, PlatformError>;
}

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("platform operation is not available")]
    Unavailable,
    #[error("platform operation failed: {0}")]
    Operation(&'static str),
}
