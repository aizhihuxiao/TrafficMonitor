//! Plugin manager for loading and managing plugins.
//!
//! Port of the original CPluginManager class.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Plugin metadata.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Plugin unique identifier
    pub id: String,
    /// Plugin display name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Plugin file path
    pub file_path: PathBuf,
    /// Whether the plugin is enabled
    pub enabled: bool,
}

/// Manages the plugin system.
///
/// Port of the original CPluginManager class.
pub struct PluginManager {
    /// Directory containing plugins
    plugins_dir: PathBuf,
    /// Loaded plugins by ID
    plugins: HashMap<String, PluginInfo>,
}

impl PluginManager {
    /// Create a new plugin manager.
    pub fn new(plugins_dir: &Path) -> Self {
        Self {
            plugins_dir: plugins_dir.to_path_buf(),
            plugins: HashMap::new(),
        }
    }

    /// Get the number of loaded plugins.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Get a plugin by ID.
    pub fn get_plugin(&self, id: &str) -> Option<&PluginInfo> {
        self.plugins.get(id)
    }

    /// Get all loaded plugins.
    pub fn plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().collect()
    }

    /// Enable or disable a plugin.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> bool {
        if let Some(plugin) = self.plugins.get_mut(id) {
            plugin.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Get the plugins directory.
    pub fn plugins_dir(&self) -> &Path {
        &self.plugins_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let dir = tempfile::tempdir().unwrap();
        let manager = PluginManager::new(dir.path());
        assert_eq!(manager.count(), 0);
    }
}
