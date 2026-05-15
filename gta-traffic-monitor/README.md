# GTA-TrafficMonitor

A cross-platform system monitoring tool written in Rust. Displays network traffic speed, CPU and memory usage, and other system information in real-time.

This is a Rust port of the original [TrafficMonitor](https://github.com/zhongyang219/TrafficMonitor) C++ application.

## Features

- **Network Traffic Monitoring**: Real-time upload and download speed
- **CPU Monitoring**: CPU usage percentage and frequency
- **Memory Monitoring**: Memory usage percentage and available memory
- **Traffic History**: Daily traffic tracking with persistence
- **Skinnable UI**: Customizable appearance via skin files
- **Plugin System**: Extensible architecture for custom monitoring items
- **Cross-Platform**: Built with Rust for cross-platform compatibility
- **Configuration**: Flexible INI and JSON-based configuration

## Project Structure

```
gta-traffic-monitor/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library root
│   ├── config/              # Configuration management
│   │   ├── ini_helper.rs    # INI file parser (port of CIniHelper)
│   │   └── settings.rs      # Settings data structures (port of CommonData)
│   ├── display/             # Display item definitions
│   │   └── items.rs         # Display items (port of DisplayItem)
│   ├── monitor/             # System monitoring
│   │   ├── cpu.rs           # CPU monitoring (port of CPUUsage)
│   │   ├── memory.rs        # Memory monitoring
│   │   ├── network.rs       # Network monitoring (port of AdapterCommon)
│   │   └── system_monitor.rs # Unified monitor (port of monitoring thread)
│   ├── skin/                # Skin system
│   │   ├── skin_file.rs     # Skin definition (port of CSkinFile)
│   │   └── skin_manager.rs  # Skin manager (port of CSkinManager)
│   ├── plugin/              # Plugin system
│   │   └── manager.rs       # Plugin manager (port of CPluginManager)
│   ├── ui/                  # User interface
│   │   ├── app.rs           # Application lifecycle (port of CTrafficMonitorApp)
│   │   └── tray.rs          # System tray icon
│   └── utils/               # Utility functions
│       ├── format.rs        # Data formatting (port of Common)
│       └── history.rs       # Traffic history (port of CHistoryTrafficFile)
├── Cargo.toml
└── README.md
```

## Module Mapping (C++ → Rust)

| Original C++ File | Rust Module |
|---|---|
| `TrafficMonitor.cpp/h` | `ui::app` |
| `TrafficMonitorDlg.cpp/h` | `ui::app`, `monitor::system_monitor` |
| `Common.cpp/h` | `utils::format` |
| `CommonData.cpp/h` | `config::settings` |
| `AdapterCommon.cpp/h` | `monitor::network` |
| `CPUUsage.cpp/h` | `monitor::cpu` |
| `IniHelper.cpp/h` | `config::ini_helper` |
| `DisplayItem.cpp/h` | `display::items` |
| `SkinFile.cpp/h` | `skin::skin_file` |
| `SkinManager.cpp/h` | `skin::skin_manager` |
| `PluginManager.cpp/h` | `plugin::manager` |
| `HistoryTrafficFile.cpp/h` | `utils::history` |
| `TaskBarDlg.cpp/h` | `ui::tray` (system tray abstraction) |

## Building

```bash
cd gta-traffic-monitor
cargo build --release
```

## Running

```bash
cargo run --release
```

The application will start monitoring system metrics and display them in the console:

```
GTA-TrafficMonitor v1.0.0
Starting system monitor...
↑ 1.50 KB/s | ↓ 25.30 KB/s | CPU: 12.5% | Mem: 45.2%
```

## Testing

```bash
cargo test
```

## Dependencies

- [sysinfo](https://crates.io/crates/sysinfo) - Cross-platform system information
- [chrono](https://crates.io/crates/chrono) - Date and time handling
- [serde](https://crates.io/crates/serde) - Serialization framework
- [serde_json](https://crates.io/crates/serde_json) - JSON serialization
- [log](https://crates.io/crates/log) / [env_logger](https://crates.io/crates/env_logger) - Logging
- [thiserror](https://crates.io/crates/thiserror) - Error handling

## License

GPL-3.0 - Same as the original TrafficMonitor project.
