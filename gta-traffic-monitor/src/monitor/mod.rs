//! System monitoring module.
//!
//! Provides cross-platform system monitoring for network traffic, CPU, memory, etc.

pub mod cpu;
pub mod memory;
pub mod network;
pub mod system_monitor;

pub use system_monitor::{MonitorSnapshot, SystemMonitor};
