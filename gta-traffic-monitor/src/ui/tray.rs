//! System tray icon management.
//!
//! Port of the notification icon functionality from CTrafficMonitorDlg.

/// System tray notification icon manager.
///
/// This is a cross-platform abstraction. Platform-specific implementations
/// would use native APIs (e.g., NOTIFYICONDATA on Windows, libappindicator on Linux).
pub struct TrayIcon {
    /// Tooltip text
    tooltip: String,
    /// Whether the icon is visible
    visible: bool,
}

impl TrayIcon {
    /// Create a new tray icon.
    pub fn new(tooltip: &str) -> Self {
        Self {
            tooltip: tooltip.to_string(),
            visible: false,
        }
    }

    /// Show the tray icon.
    pub fn show(&mut self) {
        self.visible = true;
        // Platform-specific: create/show notification icon
    }

    /// Hide the tray icon.
    pub fn hide(&mut self) {
        self.visible = false;
        // Platform-specific: remove notification icon
    }

    /// Update the tooltip text.
    pub fn set_tooltip(&mut self, tooltip: &str) {
        self.tooltip = tooltip.to_string();
        // Platform-specific: update tooltip
    }

    /// Check if the icon is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get the current tooltip text.
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tray_icon() {
        let mut icon = TrayIcon::new("GTA-TrafficMonitor");
        assert!(!icon.is_visible());
        assert_eq!(icon.tooltip(), "GTA-TrafficMonitor");

        icon.show();
        assert!(icon.is_visible());

        icon.set_tooltip("CPU: 50%");
        assert_eq!(icon.tooltip(), "CPU: 50%");

        icon.hide();
        assert!(!icon.is_visible());
    }
}
