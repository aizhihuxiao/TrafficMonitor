//! Traffic history tracking and persistence.
//!
//! Port of the original HistoryTrafficFile class.

use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Traffic data for a single day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyTraffic {
    /// Date of the traffic record
    pub date: NaiveDate,
    /// Upload bytes
    pub upload_bytes: u64,
    /// Download bytes
    pub download_bytes: u64,
}

impl DailyTraffic {
    /// Get total traffic (upload + download) in bytes.
    pub fn total_bytes(&self) -> u64 {
        self.upload_bytes + self.download_bytes
    }
}

/// View type for traffic history aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryViewType {
    Day,
    Week,
    Month,
    Year,
}

/// Manages historical traffic data.
///
/// Port of the original CHistoryTrafficFile class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficHistory {
    /// Daily traffic records indexed by date
    records: BTreeMap<String, DailyTraffic>,
}

impl TrafficHistory {
    /// Create a new empty traffic history.
    pub fn new() -> Self {
        Self {
            records: BTreeMap::new(),
        }
    }

    /// Load traffic history from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let history: TrafficHistory = serde_json::from_str(&content)?;
        Ok(history)
    }

    /// Save traffic history to a JSON file.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Add or update traffic data for today.
    pub fn update_today(&mut self, upload_bytes: u64, download_bytes: u64) {
        let today = Local::now().date_naive();
        let key = today.format("%Y-%m-%d").to_string();

        let entry = self
            .records
            .entry(key)
            .or_insert_with(|| DailyTraffic {
                date: today,
                upload_bytes: 0,
                download_bytes: 0,
            });

        entry.upload_bytes += upload_bytes;
        entry.download_bytes += download_bytes;
    }

    /// Get traffic data for a specific date.
    pub fn get_daily(&self, date: &NaiveDate) -> Option<&DailyTraffic> {
        let key = date.format("%Y-%m-%d").to_string();
        self.records.get(&key)
    }

    /// Get all traffic records.
    pub fn all_records(&self) -> Vec<&DailyTraffic> {
        self.records.values().collect()
    }

    /// Get traffic records for a specific month.
    pub fn monthly_records(&self, year: i32, month: u32) -> Vec<&DailyTraffic> {
        self.records
            .values()
            .filter(|r| r.date.year() == year && r.date.month() == month)
            .collect()
    }

    /// Get the total number of records.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Clear all records.
    pub fn clear(&mut self) {
        self.records.clear();
    }
}

impl Default for TrafficHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traffic_history_new() {
        let history = TrafficHistory::new();
        assert_eq!(history.record_count(), 0);
    }

    #[test]
    fn test_update_today() {
        let mut history = TrafficHistory::new();
        history.update_today(1024, 2048);
        assert_eq!(history.record_count(), 1);

        // Update again should accumulate
        history.update_today(512, 256);
        assert_eq!(history.record_count(), 1);

        let today = Local::now().date_naive();
        let record = history.get_daily(&today).unwrap();
        assert_eq!(record.upload_bytes, 1536);
        assert_eq!(record.download_bytes, 2304);
    }

    #[test]
    fn test_daily_traffic_total() {
        let traffic = DailyTraffic {
            date: Local::now().date_naive(),
            upload_bytes: 100,
            download_bytes: 200,
        };
        assert_eq!(traffic.total_bytes(), 300);
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("history.json");
        let path_str = path.to_str().unwrap();

        let mut history = TrafficHistory::new();
        history.update_today(1024, 2048);
        history.save_to_file(path_str).unwrap();

        let loaded = TrafficHistory::load_from_file(path_str).unwrap();
        assert_eq!(loaded.record_count(), 1);
    }
}
