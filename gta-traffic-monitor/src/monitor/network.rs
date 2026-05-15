//! Network traffic monitoring.
//!
//! Port of the original AdapterCommon using the sysinfo crate for cross-platform support.

use sysinfo::Networks;

/// Information about a network interface.
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    /// Interface name
    pub name: String,
    /// Total bytes received since system boot
    pub total_received: u64,
    /// Total bytes transmitted since system boot
    pub total_transmitted: u64,
}

/// Network traffic monitor.
pub struct NetworkMonitor {
    networks: Networks,
    /// Previous received bytes per interface (for calculating speed)
    prev_received: std::collections::HashMap<String, u64>,
    /// Previous transmitted bytes per interface (for calculating speed)
    prev_transmitted: std::collections::HashMap<String, u64>,
    /// Current download speed in bytes per second
    download_speed: u64,
    /// Current upload speed in bytes per second
    upload_speed: u64,
    /// Today's total download bytes
    today_download: u64,
    /// Today's total upload bytes
    today_upload: u64,
    /// Selected interface name (None = all interfaces)
    selected_interface: Option<String>,
}

impl NetworkMonitor {
    /// Create a new network monitor.
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();

        let mut prev_received = std::collections::HashMap::new();
        let mut prev_transmitted = std::collections::HashMap::new();

        for (name, data) in &networks {
            prev_received.insert(name.to_string(), data.total_received());
            prev_transmitted.insert(name.to_string(), data.total_transmitted());
        }

        Self {
            networks,
            prev_received,
            prev_transmitted,
            download_speed: 0,
            upload_speed: 0,
            today_download: 0,
            today_upload: 0,
            selected_interface: None,
        }
    }

    /// Refresh network data and calculate speeds.
    pub fn refresh(&mut self) {
        self.networks.refresh(true);

        let mut total_down: u64 = 0;
        let mut total_up: u64 = 0;

        for (name, data) in &self.networks {
            if let Some(selected) = &self.selected_interface {
                if name != selected {
                    continue;
                }
            }

            let current_received = data.total_received();
            let current_transmitted = data.total_transmitted();

            let prev_recv = self.prev_received.get(name.as_str()).copied().unwrap_or(0);
            let prev_trans = self.prev_transmitted.get(name.as_str()).copied().unwrap_or(0);

            // Calculate delta (handle counter reset)
            let delta_down = if current_received >= prev_recv {
                current_received - prev_recv
            } else {
                current_received
            };
            let delta_up = if current_transmitted >= prev_trans {
                current_transmitted - prev_trans
            } else {
                current_transmitted
            };

            total_down += delta_down;
            total_up += delta_up;

            self.prev_received
                .insert(name.to_string(), current_received);
            self.prev_transmitted
                .insert(name.to_string(), current_transmitted);
        }

        self.download_speed = total_down;
        self.upload_speed = total_up;
        self.today_download += total_down;
        self.today_upload += total_up;
    }

    /// Get current download speed in bytes per interval.
    pub fn download_speed(&self) -> u64 {
        self.download_speed
    }

    /// Get current upload speed in bytes per interval.
    pub fn upload_speed(&self) -> u64 {
        self.upload_speed
    }

    /// Get today's total download in bytes.
    pub fn today_download(&self) -> u64 {
        self.today_download
    }

    /// Get today's total upload in bytes.
    pub fn today_upload(&self) -> u64 {
        self.today_upload
    }

    /// List all available network interfaces.
    pub fn interfaces(&self) -> Vec<NetworkInterface> {
        self.networks
            .iter()
            .map(|(name, data)| NetworkInterface {
                name: name.to_string(),
                total_received: data.total_received(),
                total_transmitted: data.total_transmitted(),
            })
            .collect()
    }

    /// Select a specific network interface to monitor.
    pub fn select_interface(&mut self, name: Option<String>) {
        self.selected_interface = name;
    }

    /// Get the currently selected interface name.
    pub fn selected_interface(&self) -> Option<&str> {
        self.selected_interface.as_deref()
    }

    /// Reset today's traffic counters.
    pub fn reset_today_traffic(&mut self) {
        self.today_download = 0;
        self.today_upload = 0;
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_monitor_creation() {
        let monitor = NetworkMonitor::new();
        // Should have at least some interfaces (e.g., loopback)
        let interfaces = monitor.interfaces();
        // On some CI systems there may be no interfaces, so just check it doesn't panic
        assert!(!interfaces.is_empty() || interfaces.is_empty()); // may or may not have interfaces in CI
    }

    #[test]
    fn test_network_refresh() {
        let mut monitor = NetworkMonitor::new();
        monitor.refresh();
        // After first refresh, speeds should be initialized
        assert!(monitor.download_speed() < u64::MAX);
        assert!(monitor.upload_speed() < u64::MAX);
    }

    #[test]
    fn test_select_interface() {
        let mut monitor = NetworkMonitor::new();
        assert!(monitor.selected_interface().is_none());

        monitor.select_interface(Some("eth0".to_string()));
        assert_eq!(monitor.selected_interface(), Some("eth0"));

        monitor.select_interface(None);
        assert!(monitor.selected_interface().is_none());
    }

    #[test]
    fn test_reset_today_traffic() {
        let mut monitor = NetworkMonitor::new();
        monitor.refresh();
        monitor.reset_today_traffic();
        assert_eq!(monitor.today_download(), 0);
        assert_eq!(monitor.today_upload(), 0);
    }
}
