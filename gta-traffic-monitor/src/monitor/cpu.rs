//! CPU usage monitoring.
//!
//! Port of the original CPUUsage.h/cpp using the sysinfo crate.

use sysinfo::System;

/// CPU usage monitor.
pub struct CpuMonitor {
    system: System,
}

impl CpuMonitor {
    /// Create a new CPU monitor.
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu_all();
        Self { system }
    }

    /// Refresh CPU data and return overall CPU usage percentage.
    pub fn refresh(&mut self) -> f32 {
        self.system.refresh_cpu_all();
        self.system.global_cpu_usage()
    }

    /// Get per-core CPU usage percentages.
    pub fn per_core_usage(&self) -> Vec<f32> {
        self.system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
    }

    /// Get the number of CPU cores.
    pub fn cpu_count(&self) -> usize {
        self.system.cpus().len()
    }

    /// Get CPU brand name.
    pub fn cpu_brand(&self) -> String {
        self.system
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_default()
    }

    /// Get CPU frequency in MHz.
    pub fn cpu_frequency(&self) -> u64 {
        self.system
            .cpus()
            .first()
            .map(|cpu| cpu.frequency())
            .unwrap_or(0)
    }
}

impl Default for CpuMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_monitor_creation() {
        let monitor = CpuMonitor::new();
        assert!(monitor.cpu_count() > 0);
    }

    #[test]
    fn test_cpu_monitor_refresh() {
        let mut monitor = CpuMonitor::new();
        // First refresh might return 0 as sysinfo needs two data points
        std::thread::sleep(std::time::Duration::from_millis(200));
        let usage = monitor.refresh();
        // Usage should be between 0 and 100
        assert!(usage >= 0.0 && usage <= 100.0);
    }

    #[test]
    fn test_cpu_brand() {
        let monitor = CpuMonitor::new();
        let brand = monitor.cpu_brand();
        // Brand should not be empty on most systems
        assert!(!brand.is_empty() || monitor.cpu_count() == 0);
    }
}
