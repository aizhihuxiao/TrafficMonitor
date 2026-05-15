//! GTA-TrafficMonitor - System monitoring tool
//!
//! Displays network traffic, CPU usage, memory usage and other system metrics.

use gta_traffic_monitor::config::AppConfig;
use gta_traffic_monitor::monitor::SystemMonitor;
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    info!("GTA-TrafficMonitor v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting system monitor...");

    let config = AppConfig::default();
    let monitor = Arc::new(Mutex::new(SystemMonitor::new(config.general.monitor_time_span)));

    // Initialize monitor
    {
        let mut mon = monitor.lock().unwrap();
        mon.refresh();
        info!("System monitor initialized");
    }

    let monitor_thread = {
        let monitor = Arc::clone(&monitor);
        let interval = Duration::from_millis(config.general.monitor_time_span as u64);
        thread::spawn(move || loop {
            thread::sleep(interval);
            let mut mon = monitor.lock().unwrap();
            mon.refresh();

            let snapshot = mon.snapshot();
            info!(
                "↑ {} | ↓ {} | CPU: {:.1}% | Mem: {:.1}%",
                gta_traffic_monitor::utils::format::data_size_to_string(snapshot.upload_speed),
                gta_traffic_monitor::utils::format::data_size_to_string(snapshot.download_speed),
                snapshot.cpu_usage,
                snapshot.memory_usage,
            );
        })
    };

    info!("Monitor thread started. Press Ctrl+C to exit.");
    monitor_thread.join().unwrap();
}
