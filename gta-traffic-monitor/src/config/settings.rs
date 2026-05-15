//! Application settings and configuration data structures.
//!
//! Port of the original CommonData.h/cpp configuration structures.

use serde::{Deserialize, Serialize};

/// Speed display unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedUnit {
    /// Automatically choose appropriate unit
    Auto,
    /// Kilobytes per second
    KBps,
    /// Megabytes per second
    MBps,
}

impl Default for SpeedUnit {
    fn default() -> Self {
        Self::Auto
    }
}

/// CPU usage acquisition method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CpuUsageMethod {
    /// Use system CPU times
    CpuTime,
    /// Use platform-specific performance counters
    PerformanceCounter,
    /// Use hardware monitor library
    HardwareMonitor,
}

impl Default for CpuUsageMethod {
    fn default() -> Self {
        Self::CpuTime
    }
}

/// Memory display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryDisplay {
    /// Show usage percentage
    UsagePercentage,
    /// Show available memory in MB/GB
    AvailableMemory,
    /// Show used memory in MB/GB
    UsedMemory,
}

impl Default for MemoryDisplay {
    fn default() -> Self {
        Self::UsagePercentage
    }
}

/// Action to perform on double-click.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DoubleClickAction {
    /// Open network connections dialog
    OpenConnectionDialog,
    /// Open options/settings
    OpenOptions,
    /// Open task manager
    OpenTaskManager,
    /// Open specified application
    OpenSpecifiedApp,
    /// Do nothing
    None,
}

impl Default for DoubleClickAction {
    fn default() -> Self {
        Self::OpenConnectionDialog
    }
}

/// Color represented as RGB values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_rgb(rgb: u32) -> Self {
        Self {
            r: (rgb & 0xFF) as u8,
            g: ((rgb >> 8) & 0xFF) as u8,
            b: ((rgb >> 16) & 0xFF) as u8,
        }
    }

    pub fn to_rgb(&self) -> u32 {
        (self.r as u32) | ((self.g as u32) << 8) | ((self.b as u32) << 16)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}

/// Font information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontInfo {
    pub name: String,
    pub size: i32,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikeout: bool,
}

impl Default for FontInfo {
    fn default() -> Self {
        Self {
            name: String::from("Microsoft YaHei"),
            size: 9,
            bold: false,
            italic: false,
            underline: false,
            strikeout: false,
        }
    }
}

/// General application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Language identifier (0 = follow system)
    pub language: u16,
    /// CPU usage acquisition method
    pub cpu_usage_method: CpuUsageMethod,
    /// Monitor refresh interval in milliseconds
    pub monitor_time_span: u32,
    /// Whether to check for updates on startup
    pub check_update_on_start: bool,
    /// Whether to auto-run on system startup
    pub auto_run: bool,
    /// Whether to allow multiple instances
    pub allow_multiple_instances: bool,
    /// Whether to show notification area icon
    pub show_notify_icon: bool,
    /// Double-click action for main window
    pub double_click_action: DoubleClickAction,
    /// Speed unit
    pub speed_unit: SpeedUnit,
    /// Memory display mode
    pub memory_display: MemoryDisplay,
    /// Number of decimal places for speed display
    pub speed_decimals: u8,
    /// Whether to show all network adapters
    pub select_all_adapters: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            language: 0,
            cpu_usage_method: CpuUsageMethod::default(),
            monitor_time_span: 1000,
            check_update_on_start: true,
            auto_run: false,
            allow_multiple_instances: false,
            show_notify_icon: true,
            double_click_action: DoubleClickAction::default(),
            speed_unit: SpeedUnit::default(),
            memory_display: MemoryDisplay::default(),
            speed_decimals: 1,
            select_all_adapters: false,
        }
    }
}

/// Main window appearance settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MainWndSettings {
    /// Window transparency (0-100)
    pub transparency: u8,
    /// Current skin name
    pub skin_name: String,
    /// Whether window is always on top
    pub always_on_top: bool,
    /// Whether window position is locked
    pub lock_window_pos: bool,
    /// Whether mouse can click through the window
    pub mouse_penetrate: bool,
    /// Whether to show the taskbar window
    pub show_taskbar_wnd: bool,
    /// Text color for display items
    pub text_colors: Vec<Color>,
    /// Background color
    pub background_color: Color,
    /// Font information
    pub font: FontInfo,
    /// Whether to swap upload/download display order
    pub swap_up_down: bool,
    /// Whether to show the main window
    pub show_main_wnd: bool,
}

impl Default for MainWndSettings {
    fn default() -> Self {
        Self {
            transparency: 100,
            skin_name: String::from("default"),
            always_on_top: false,
            lock_window_pos: false,
            mouse_penetrate: false,
            show_taskbar_wnd: true,
            text_colors: vec![Color::new(0, 255, 0); 13],
            background_color: Color::black(),
            font: FontInfo::default(),
            swap_up_down: false,
            show_main_wnd: true,
        }
    }
}

/// Taskbar window settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBarSettings {
    /// Background color
    pub background_color: Color,
    /// Text color
    pub text_colors: Vec<Color>,
    /// Whether to use transparent background
    pub transparent_color_enabled: bool,
    /// Transparent color
    pub transparent_color: Color,
    /// Font information
    pub font: FontInfo,
    /// Whether to show the graph
    pub show_graph: bool,
    /// Graph bar color
    pub graph_color: Color,
}

impl Default for TaskBarSettings {
    fn default() -> Self {
        Self {
            background_color: Color::black(),
            text_colors: vec![Color::new(0, 255, 0); 13],
            transparent_color_enabled: false,
            transparent_color: Color::black(),
            font: FontInfo::default(),
            show_graph: false,
            graph_color: Color::new(0, 255, 0),
        }
    }
}

/// Complete application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralSettings,
    pub main_wnd: MainWndSettings,
    pub taskbar: TaskBarSettings,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            main_wnd: MainWndSettings::default(),
            taskbar: TaskBarSettings::default(),
        }
    }
}

impl AppConfig {
    /// Load configuration from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a JSON file.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.general.monitor_time_span, 1000);
        assert_eq!(config.main_wnd.transparency, 100);
        assert!(config.general.show_notify_icon);
    }

    #[test]
    fn test_color_conversion() {
        let color = Color::new(255, 128, 0);
        let rgb = color.to_rgb();
        let back = Color::from_rgb(rgb);
        assert_eq!(color, back);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.general.monitor_time_span,
            config.general.monitor_time_span
        );
    }

    #[test]
    fn test_config_save_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        let path_str = path.to_str().unwrap();

        let config = AppConfig::default();
        config.save_to_file(path_str).unwrap();

        let loaded = AppConfig::load_from_file(path_str).unwrap();
        assert_eq!(
            loaded.general.monitor_time_span,
            config.general.monitor_time_span
        );
        assert_eq!(loaded.main_wnd.transparency, config.main_wnd.transparency);
    }
}
