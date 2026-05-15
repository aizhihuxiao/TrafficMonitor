//! Data formatting utilities.
//!
//! Port of the formatting functions from Common.h/cpp.

/// Format a byte count as a human-readable data size string.
///
/// Port of DataSizeToString from Common.cpp.
pub fn data_size_to_string(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const TB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= TB {
        format!("{:.2} TB/s", bytes_f / TB)
    } else if bytes_f >= GB {
        format!("{:.2} GB/s", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB/s", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB/s", bytes_f / KB)
    } else {
        format!("{} B/s", bytes)
    }
}

/// Format a byte count as a human-readable size string (without /s).
pub fn bytes_to_string(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const TB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= TB {
        format!("{:.2} TB", bytes_f / TB)
    } else if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

/// Format a temperature value as a string.
///
/// Port of TemperatureToString from Common.cpp.
pub fn temperature_to_string(temp: f32) -> String {
    format!("{:.0}°C", temp)
}

/// Format a usage percentage as a string.
///
/// Port of UsageToString from Common.cpp.
pub fn usage_to_string(usage: f32) -> String {
    format!("{:.1}%", usage)
}

/// Format CPU frequency in MHz/GHz.
///
/// Port of FreqToString from Common.cpp.
pub fn freq_to_string(freq_mhz: u64) -> String {
    if freq_mhz >= 1000 {
        format!("{:.2} GHz", freq_mhz as f64 / 1000.0)
    } else {
        format!("{} MHz", freq_mhz)
    }
}

/// Format kilobytes as a human-readable string.
///
/// Port of KBytesToString from Common.cpp.
pub fn kbytes_to_string(kbytes: u64) -> String {
    bytes_to_string(kbytes * 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_size_to_string() {
        assert_eq!(data_size_to_string(0), "0 B/s");
        assert_eq!(data_size_to_string(512), "512 B/s");
        assert_eq!(data_size_to_string(1024), "1.00 KB/s");
        assert_eq!(data_size_to_string(1048576), "1.00 MB/s");
        assert_eq!(data_size_to_string(1073741824), "1.00 GB/s");
    }

    #[test]
    fn test_bytes_to_string() {
        assert_eq!(bytes_to_string(0), "0 B");
        assert_eq!(bytes_to_string(1024), "1.00 KB");
        assert_eq!(bytes_to_string(1048576), "1.00 MB");
    }

    #[test]
    fn test_temperature_to_string() {
        assert_eq!(temperature_to_string(45.5), "46°C");
        assert_eq!(temperature_to_string(0.0), "0°C");
    }

    #[test]
    fn test_usage_to_string() {
        assert_eq!(usage_to_string(50.0), "50.0%");
        assert_eq!(usage_to_string(99.9), "99.9%");
    }

    #[test]
    fn test_freq_to_string() {
        assert_eq!(freq_to_string(800), "800 MHz");
        assert_eq!(freq_to_string(3500), "3.50 GHz");
    }

    #[test]
    fn test_kbytes_to_string() {
        assert_eq!(kbytes_to_string(1024), "1.00 MB");
    }
}
