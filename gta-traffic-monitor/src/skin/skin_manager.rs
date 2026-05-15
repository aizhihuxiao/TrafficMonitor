//! Skin manager for loading and switching between skins.
//!
//! Port of the original CSkinManager class.

use super::skin_file::SkinFile;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Manages available skins and the current active skin.
pub struct SkinManager {
    /// Directory containing skins
    skins_dir: PathBuf,
    /// Loaded skins by name
    skins: HashMap<String, SkinFile>,
    /// Currently active skin name
    current_skin: String,
}

impl SkinManager {
    /// Create a new skin manager with the specified skins directory.
    pub fn new(skins_dir: &Path) -> Self {
        Self {
            skins_dir: skins_dir.to_path_buf(),
            skins: HashMap::new(),
            current_skin: String::from("default"),
        }
    }

    /// Load all skins from the skins directory.
    pub fn load_skins(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        self.skins.clear();

        // Always add the default skin
        self.skins
            .insert("default".to_string(), SkinFile::default());

        if !self.skins_dir.exists() {
            return Ok(1);
        }

        let entries = std::fs::read_dir(&self.skins_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(skin) = SkinFile::load_from_file(path.to_str().unwrap_or_default()) {
                    self.skins.insert(skin.name.clone(), skin);
                }
            }
        }

        Ok(self.skins.len())
    }

    /// Get the current active skin.
    pub fn current_skin(&self) -> Option<&SkinFile> {
        self.skins.get(&self.current_skin)
    }

    /// Set the current active skin by name.
    pub fn set_current_skin(&mut self, name: &str) -> bool {
        if self.skins.contains_key(name) {
            self.current_skin = name.to_string();
            true
        } else {
            false
        }
    }

    /// Get a list of available skin names.
    pub fn available_skins(&self) -> Vec<&str> {
        self.skins.keys().map(|s| s.as_str()).collect()
    }

    /// Get a skin by name.
    pub fn get_skin(&self, name: &str) -> Option<&SkinFile> {
        self.skins.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skin_manager_creation() {
        let dir = tempfile::tempdir().unwrap();
        let manager = SkinManager::new(dir.path());
        assert_eq!(manager.current_skin, "default");
    }

    #[test]
    fn test_load_skins_with_default() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = SkinManager::new(dir.path());
        let count = manager.load_skins().unwrap();
        assert!(count >= 1); // At least the default skin
        assert!(manager.current_skin().is_some());
    }

    #[test]
    fn test_set_current_skin() {
        let dir = tempfile::tempdir().unwrap();
        let mut manager = SkinManager::new(dir.path());
        manager.load_skins().unwrap();

        assert!(manager.set_current_skin("default"));
        assert!(!manager.set_current_skin("nonexistent"));
    }
}
