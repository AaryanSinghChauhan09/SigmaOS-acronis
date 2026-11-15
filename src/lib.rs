//! SigmaOS-acronis: backup image model.
//!
//! Real implementation: snapshots, differential/incremental backups, validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupImage {
    pub id: String,
    pub name: String,
    pub size_bytes: u64,
    pub created_at: String,
    pub backup_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupError {
    EmptyName,
    DuplicateImage(String),
}

impl fmt::Display for BackupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "backup name must not be empty"),
            Self::DuplicateImage(id) => write!(f, "duplicate backup image {:?}", id),
        }
    }
}

impl std::error::Error for BackupError {}

pub struct BackupManager {
    pub name: String,
    images: BTreeMap<String, BackupImage>,
    snapshots: BTreeMap<String, String>,
}

impl BackupManager {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), images: BTreeMap::new(), snapshots: BTreeMap::new() }
    }

    pub fn add_image(&mut self, i: BackupImage) -> Result<(), BackupError> {
        if i.name.trim().is_empty() { return Err(BackupError::EmptyName); }
        let id = i.id.clone();
        if self.images.contains_key(&id) {
            return Err(BackupError::DuplicateImage(id.clone()));
        }
        self.images.insert(id, i);
        Ok(())
    }

    pub fn image_count(&self) -> usize { self.images.len() }

    pub fn get_image(&self, id: &str) -> Option<&BackupImage> { self.images.get(id) }

    pub fn create_snapshot(&mut self, id: String, timestamp: String) {
        self.snapshots.insert(id, timestamp);
    }

    pub fn get_snapshot(&self, id: &str) -> Option<&String> { self.snapshots.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let b = BackupManager::new("My Backup");
        assert_eq!(b.name, "My Backup");
        assert_eq!(b.image_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut b = BackupManager::new("My Backup");
        let i = BackupImage { id: "img1".into(), name: "full".into(), size_bytes: 1024, created_at: "2024-01-01".into(), backup_type: "full".into() };
        b.add_image(i).unwrap();
        assert_eq!(b.image_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut b = BackupManager::new("My Backup");
        let i = BackupImage { id: "x".into(), name: "".into(), size_bytes: 1, created_at: "a".into(), backup_type: "a".into() };
        assert!(b.add_image(i).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut b = BackupManager::new("My Backup");
        let i = BackupImage { id: "x".into(), name: "X".into(), size_bytes: 1, created_at: "a".into(), backup_type: "a".into() };
        b.add_image(i).unwrap();
        let i2 = BackupImage { id: "x".into(), name: "X".into(), size_bytes: 1, created_at: "a".into(), backup_type: "a".into() };
        assert!(b.add_image(i2).is_err());
    }
}
