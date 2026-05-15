//! Memory usage monitoring.

use sysinfo::System;

/// Memory usage monitor.
pub struct MemoryMonitor {
    system: System,
}

impl MemoryMonitor {
    /// Create a new memory monitor.
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_memory();
        Self { system }
    }

    /// Refresh memory data.
    pub fn refresh(&mut self) {
        self.system.refresh_memory();
    }

    /// Get total physical memory in bytes.
    pub fn total_memory(&self) -> u64 {
        self.system.total_memory()
    }

    /// Get used physical memory in bytes.
    pub fn used_memory(&self) -> u64 {
        self.system.used_memory()
    }

    /// Get available physical memory in bytes.
    pub fn available_memory(&self) -> u64 {
        self.system.available_memory()
    }

    /// Get memory usage as a percentage (0-100).
    pub fn usage_percentage(&self) -> f32 {
        let total = self.total_memory();
        if total == 0 {
            return 0.0;
        }
        (self.used_memory() as f64 / total as f64 * 100.0) as f32
    }

    /// Get total swap memory in bytes.
    pub fn total_swap(&self) -> u64 {
        self.system.total_swap()
    }

    /// Get used swap memory in bytes.
    pub fn used_swap(&self) -> u64 {
        self.system.used_swap()
    }
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_monitor_creation() {
        let monitor = MemoryMonitor::new();
        assert!(monitor.total_memory() > 0);
    }

    #[test]
    fn test_memory_usage() {
        let monitor = MemoryMonitor::new();
        let usage = monitor.usage_percentage();
        assert!(usage >= 0.0 && usage <= 100.0);
    }

    #[test]
    fn test_used_less_than_total() {
        let monitor = MemoryMonitor::new();
        assert!(monitor.used_memory() <= monitor.total_memory());
    }
}
