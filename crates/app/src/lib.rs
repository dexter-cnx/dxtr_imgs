use dxtr_imgs_domain::{Asset, AssetId, Workplace, WorkplaceId};
use thiserror::Error;

pub trait WorkplaceRepository: Send + Sync {
    fn ensure_default(&self) -> Result<Workplace, RepositoryError>;
    fn active_workplace(&self) -> Result<Option<Workplace>, RepositoryError>;
    fn set_active_workplace(&self, id: &WorkplaceId) -> Result<(), RepositoryError>;
}

pub trait CatalogRepository: Send + Sync {
    fn assets(&self, workplace_id: &WorkplaceId) -> Result<Vec<Asset>, RepositoryError>;
    fn asset(&self, id: &AssetId) -> Result<Option<Asset>, RepositoryError>;
    fn upsert_asset(&self, asset: &Asset) -> Result<(), RepositoryError>;
    fn remove_from_catalog(&self, id: &AssetId) -> Result<(), RepositoryError>;
}

pub trait ImportBatchRepository: Send + Sync {}
pub trait SettingsRepository: Send + Sync {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MessageKey {
    AppName,
    Workplace,
    Catalog,
    AllPhotos,
    Missing,
    RecentImports,
    Develop,
}

pub trait Translator: Send + Sync {
    fn tr(&self, key: MessageKey) -> &'static str;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct EnglishTranslator;

impl Translator for EnglishTranslator {
    fn tr(&self, key: MessageKey) -> &'static str {
        match key {
            MessageKey::AppName => "Dextryx Images",
            MessageKey::Workplace => "Workplace",
            MessageKey::Catalog => "Catalog",
            MessageKey::AllPhotos => "All photos",
            MessageKey::Missing => "Missing",
            MessageKey::RecentImports => "Recent imports",
            MessageKey::Develop => "Develop",
        }
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("repository unavailable")]
    Unavailable,
    #[error("repository invariant violated: {0}")]
    Invariant(&'static str),
}
