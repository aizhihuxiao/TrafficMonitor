//! Main application lifecycle management.
//!
//! Port of the original CTrafficMonitorApp and CTrafficMonitorDlg classes.

use crate::config::AppConfig;
use crate::monitor::SystemMonitor;
use crate::plugin::PluginManager;
use crate::skin::SkinManager;
use std::path::PathBuf;

/// Application state and lifecycle manager.
///
/// Port of the original CTrafficMonitorApp class.
pub struct App {
    /// Application configuration
    pub config: AppConfig,
    /// System monitor
    pub monitor: SystemMonitor,
    /// Skin manager
    pub skin_manager: SkinManager,
    /// Plugin manager
    pub plugin_manager: PluginManager,
    /// Application data directory
    pub data_dir: PathBuf,
    /// Whether the application is running
    pub running: bool,
}

impl App {
    /// Create a new application instance.
    pub fn new() -> Self {
        let data_dir = Self::default_data_dir();

        let config = Self::load_config(&data_dir);
        let monitor = SystemMonitor::new(config.general.monitor_time_span);
        let skin_manager = SkinManager::new(&data_dir.join("skins"));
        let plugin_manager = PluginManager::new(&data_dir.join("plugins"));

        Self {
            config,
            monitor,
            skin_manager,
            plugin_manager,
            data_dir,
            running: false,
        }
    }

    /// Get the default data directory.
    fn default_data_dir() -> PathBuf {
        dirs_candidate().unwrap_or_else(|| PathBuf::from("."))
    }

    /// Load configuration from the data directory.
    fn load_config(data_dir: &Path) -> AppConfig {
        let config_path = data_dir.join("config.json");
        if config_path.exists() {
            AppConfig::load_from_file(config_path.to_str().unwrap_or_default()).unwrap_or_default()
        } else {
            AppConfig::default()
        }
    }

    /// Save current configuration.
    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.data_dir.join("config.json");
        std::fs::create_dir_all(&self.data_dir)?;
        self.config.save_to_file(config_path.to_str().unwrap_or_default())
    }

    /// Initialize the application.
    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load skins
        self.skin_manager.load_skins()?;

        // Set the current skin from config
        self.skin_manager
            .set_current_skin(&self.config.main_wnd.skin_name);

        // Initialize monitor
        self.monitor.refresh();

        self.running = true;
        Ok(())
    }

    /// Get application version string.
    pub fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Get application name.
    pub fn name() -> &'static str {
        "GTA-TrafficMonitor"
    }
}

use std::path::Path;

/// Get candidate directory for application data.
fn dirs_candidate() -> Option<PathBuf> {
    // Use current directory + app name
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    exe_dir.map(|d| d.join("GTA-TrafficMonitor"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_name_and_version() {
        assert_eq!(App::name(), "GTA-TrafficMonitor");
        assert!(!App::version().is_empty());
    }
}
