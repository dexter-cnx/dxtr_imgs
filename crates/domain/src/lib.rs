use std::path::{Path, PathBuf};

use thiserror::Error;
use uuid::Uuid;

pub const DEFAULT_WORKPLACE_NAME: &str = "My workplace";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorkplaceId(Uuid);

impl WorkplaceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WorkplaceId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AssetId(Uuid);

impl AssetId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for AssetId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageMode {
    Linked,
    Managed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaType {
    Raster,
    Raw,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workplace {
    pub id: WorkplaceId,
    pub name: String,
}

impl Workplace {
    pub fn default_workplace() -> Self {
        Self {
            id: WorkplaceId::new(),
            name: DEFAULT_WORKPLACE_NAME.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asset {
    pub id: AssetId,
    pub workplace_id: WorkplaceId,
    pub storage_mode: StorageMode,
    pub source_path: PathBuf,
    pub managed_path: Option<PathBuf>,
    pub media_type: MediaType,
    pub missing: bool,
}

impl Asset {
    pub fn effective_path(&self) -> Result<&Path, DomainError> {
        match self.storage_mode {
            StorageMode::Linked => Ok(self.source_path.as_path()),
            StorageMode::Managed => self
                .managed_path
                .as_deref()
                .ok_or(DomainError::ManagedAssetWithoutManagedPath),
        }
    }

    pub fn relink(&mut self, path: PathBuf) {
        match self.storage_mode {
            StorageMode::Linked => self.source_path = path,
            StorageMode::Managed => self.managed_path = Some(path),
        }
        self.missing = false;
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("managed asset is missing its managed path")]
    ManagedAssetWithoutManagedPath,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn linked_asset() -> Asset {
        Asset {
            id: AssetId::new(),
            workplace_id: WorkplaceId::new(),
            storage_mode: StorageMode::Linked,
            source_path: PathBuf::from("/photos/original.jpg"),
            managed_path: None,
            media_type: MediaType::Raster,
            missing: true,
        }
    }

    #[test]
    fn default_workplace_has_expected_name() {
        assert_eq!(Workplace::default_workplace().name, DEFAULT_WORKPLACE_NAME);
    }

    #[test]
    fn linked_asset_uses_source_path() {
        let asset = linked_asset();
        assert_eq!(
            asset.effective_path().unwrap(),
            Path::new("/photos/original.jpg")
        );
    }

    #[test]
    fn relink_preserves_asset_identity() {
        let mut asset = linked_asset();
        let id = asset.id.clone();

        asset.relink(PathBuf::from("/photos/relinked.jpg"));

        assert_eq!(asset.id, id);
        assert_eq!(asset.source_path, PathBuf::from("/photos/relinked.jpg"));
        assert!(!asset.missing);
    }

    #[test]
    fn managed_asset_requires_managed_path() {
        let mut asset = linked_asset();
        asset.storage_mode = StorageMode::Managed;
        assert_eq!(
            asset.effective_path(),
            Err(DomainError::ManagedAssetWithoutManagedPath)
        );
    }
}
