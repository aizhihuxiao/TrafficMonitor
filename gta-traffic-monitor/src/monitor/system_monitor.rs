//! Unified system monitor that combines all monitoring subsystems.
//!
//! Port of the monitoring logic from CTrafficMonitorDlg::DoMonitorAcquisition.

use super::cpu::CpuMonitor;
use super::memory::MemoryMonitor;
use super::network::NetworkMonitor;

/// A snapshot of all monitored system metrics at a point in time.
#[derive(Debug, Clone)]
pub struct MonitorSnapshot {
    /// Upload speed in bytes per interval
    pub upload_speed: u64,
    /// Download speed in bytes per interval
    pub download_speed: u64,
    /// Total speed (upload + download)
    pub total_speed: u64,
    /// CPU usage percentage (0-100)
    pub cpu_usage: f32,
    /// Memory usage percentage (0-100)
    pub memory_usage: f32,
    /// Total memory in bytes
    pub total_memory: u64,
    /// Used memory in bytes
    pub used_memory: u64,
    /// Available memory in bytes
    pub available_memory: u64,
    /// CPU frequency in MHz
    pub cpu_frequency: u64,
    /// Today's total download in bytes
    pub today_download: u64,
    /// Today's total upload in bytes
    pub today_upload: u64,
}

impl Default for MonitorSnapshot {
    fn default() -> Self {
        Self {
            upload_speed: 0,
            download_speed: 0,
            total_speed: 0,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            total_memory: 0,
            used_memory: 0,
            available_memory: 0,
            cpu_frequency: 0,
            today_download: 0,
            today_upload: 0,
        }
    }
}

/// Unified system monitor combining network, CPU, and memory monitoring.
///
/// Port of the monitoring thread logic from CTrafficMonitorDlg.
pub struct SystemMonitor {
    cpu: CpuMonitor,
    memory: MemoryMonitor,
    network: NetworkMonitor,
    snapshot: MonitorSnapshot,
    /// Monitor refresh interval in milliseconds
    interval_ms: u32,
}

impl SystemMonitor {
    /// Create a new system monitor with the specified refresh interval.
    pub fn new(interval_ms: u32) -> Self {
        Self {
            cpu: CpuMonitor::new(),
            memory: MemoryMonitor::new(),
            network: NetworkMonitor::new(),
            snapshot: MonitorSnapshot::default(),
            interval_ms,
        }
    }

    /// Refresh all monitoring data.
    pub fn refresh(&mut self) {
        // Refresh all subsystems
        let cpu_usage = self.cpu.refresh();
        self.memory.refresh();
        self.network.refresh();

        // Build snapshot
        let download = self.network.download_speed();
        let upload = self.network.upload_speed();

        self.snapshot = MonitorSnapshot {
            upload_speed: upload,
            download_speed: download,
            total_speed: upload + download,
            cpu_usage,
            memory_usage: self.memory.usage_percentage(),
            total_memory: self.memory.total_memory(),
            used_memory: self.memory.used_memory(),
            available_memory: self.memory.available_memory(),
            cpu_frequency: self.cpu.cpu_frequency(),
            today_download: self.network.today_download(),
            today_upload: self.network.today_upload(),
        };
    }

    /// Get the current snapshot of all metrics.
    pub fn snapshot(&self) -> &MonitorSnapshot {
        &self.snapshot
    }

    /// Get a reference to the network monitor.
    pub fn network(&self) -> &NetworkMonitor {
        &self.network
    }

    /// Get a mutable reference to the network monitor.
    pub fn network_mut(&mut self) -> &mut NetworkMonitor {
        &mut self.network
    }

    /// Get the refresh interval in milliseconds.
    pub fn interval_ms(&self) -> u32 {
        self.interval_ms
    }

    /// Set the refresh interval in milliseconds.
    pub fn set_interval_ms(&mut self, ms: u32) {
        self.interval_ms = ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_creation() {
        let monitor = SystemMonitor::new(1000);
        assert_eq!(monitor.interval_ms(), 1000);
    }

    #[test]
    fn test_system_monitor_refresh() {
        let mut monitor = SystemMonitor::new(1000);
        monitor.refresh();

        let snapshot = monitor.snapshot();
        assert!(snapshot.total_memory > 0);
        assert!(snapshot.memory_usage >= 0.0 && snapshot.memory_usage <= 100.0);
    }

    #[test]
    fn test_snapshot_default() {
        let snapshot = MonitorSnapshot::default();
        assert_eq!(snapshot.upload_speed, 0);
        assert_eq!(snapshot.download_speed, 0);
        assert_eq!(snapshot.cpu_usage, 0.0);
    }
}
