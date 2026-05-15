//! Display item types and operations.
//!
//! Defines the items that can be displayed in the monitoring UI.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Built-in display items corresponding to the original C++ DisplayItem enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DisplayItem {
    /// Upload speed
    Upload,
    /// Download speed
    Download,
    /// CPU usage percentage
    CpuUsage,
    /// Memory usage
    MemoryUsage,
    /// GPU usage percentage
    GpuUsage,
    /// CPU temperature
    CpuTemperature,
    /// GPU temperature
    GpuTemperature,
    /// HDD temperature
    HddTemperature,
    /// Mainboard temperature
    MainboardTemperature,
    /// HDD usage percentage
    HddUsage,
    /// Total speed (upload + download)
    TotalSpeed,
    /// CPU frequency
    CpuFrequency,
    /// Today's total traffic
    TodayTraffic,
}

impl DisplayItem {
    /// Get all built-in display items.
    pub fn all() -> &'static [DisplayItem] {
        &[
            DisplayItem::Upload,
            DisplayItem::Download,
            DisplayItem::CpuUsage,
            DisplayItem::MemoryUsage,
            DisplayItem::GpuUsage,
            DisplayItem::CpuTemperature,
            DisplayItem::GpuTemperature,
            DisplayItem::HddTemperature,
            DisplayItem::MainboardTemperature,
            DisplayItem::HddUsage,
            DisplayItem::TotalSpeed,
            DisplayItem::CpuFrequency,
            DisplayItem::TodayTraffic,
        ]
    }

    /// Get the INI key name for this item.
    pub fn ini_key(&self) -> &'static str {
        match self {
            DisplayItem::Upload => "up",
            DisplayItem::Download => "down",
            DisplayItem::CpuUsage => "cpu",
            DisplayItem::MemoryUsage => "memory",
            DisplayItem::GpuUsage => "gpu_usage",
            DisplayItem::CpuTemperature => "cpu_temp",
            DisplayItem::GpuTemperature => "gpu_temp",
            DisplayItem::HddTemperature => "hdd_temp",
            DisplayItem::MainboardTemperature => "main_board_temp",
            DisplayItem::HddUsage => "hdd_usage",
            DisplayItem::TotalSpeed => "total_speed",
            DisplayItem::CpuFrequency => "cpu_freq",
            DisplayItem::TodayTraffic => "today_traffic",
        }
    }

    /// Get the default label text for this item.
    pub fn default_label(&self) -> &'static str {
        match self {
            DisplayItem::Upload => "↑:",
            DisplayItem::Download => "↓:",
            DisplayItem::CpuUsage => "CPU:",
            DisplayItem::MemoryUsage => "MEM:",
            DisplayItem::GpuUsage => "GPU:",
            DisplayItem::CpuTemperature => "CPU Temp:",
            DisplayItem::GpuTemperature => "GPU Temp:",
            DisplayItem::HddTemperature => "HDD Temp:",
            DisplayItem::MainboardTemperature => "MB Temp:",
            DisplayItem::HddUsage => "HDD:",
            DisplayItem::TotalSpeed => "Total:",
            DisplayItem::CpuFrequency => "Freq:",
            DisplayItem::TodayTraffic => "Today:",
        }
    }
}

impl fmt::Display for DisplayItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.default_label())
    }
}

/// A display item that can be either a built-in item or a plugin-provided item.
///
/// Port of the original CommonDisplayItem class.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommonDisplayItem {
    /// A built-in display item.
    BuiltIn(DisplayItem),
    /// A plugin-provided display item with a unique identifier.
    Plugin {
        id: String,
        name: String,
    },
}

impl CommonDisplayItem {
    /// Check if this is a plugin item.
    pub fn is_plugin(&self) -> bool {
        matches!(self, CommonDisplayItem::Plugin { .. })
    }

    /// Get the display name.
    pub fn name(&self) -> String {
        match self {
            CommonDisplayItem::BuiltIn(item) => item.default_label().to_string(),
            CommonDisplayItem::Plugin { name, .. } => name.clone(),
        }
    }
}

/// A set of display items with bitmask serialization.
///
/// Port of the original DisplayItemSet class.
#[derive(Debug, Clone, Default)]
pub struct DisplayItemSet {
    items: std::collections::HashSet<DisplayItem>,
}

impl DisplayItemSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, item: DisplayItem) {
        self.items.insert(item);
    }

    pub fn remove(&mut self, item: DisplayItem) {
        self.items.remove(&item);
    }

    pub fn contains(&self, item: &DisplayItem) -> bool {
        self.items.contains(item)
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Serialize to an integer bitmask.
    pub fn to_bitmask(&self) -> u32 {
        let mut mask = 0u32;
        for item in DisplayItem::all() {
            if self.items.contains(item) {
                let idx = DisplayItem::all().iter().position(|i| i == item).unwrap();
                mask |= 1 << idx;
            }
        }
        mask
    }

    /// Deserialize from an integer bitmask.
    pub fn from_bitmask(mask: u32) -> Self {
        let mut set = Self::new();
        for (idx, item) in DisplayItem::all().iter().enumerate() {
            if mask & (1 << idx) != 0 {
                set.add(*item);
            }
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_item_all() {
        assert_eq!(DisplayItem::all().len(), 13);
    }

    #[test]
    fn test_display_item_set_bitmask() {
        let mut set = DisplayItemSet::new();
        set.add(DisplayItem::Upload);
        set.add(DisplayItem::Download);
        set.add(DisplayItem::CpuUsage);

        let mask = set.to_bitmask();
        let restored = DisplayItemSet::from_bitmask(mask);

        assert!(restored.contains(&DisplayItem::Upload));
        assert!(restored.contains(&DisplayItem::Download));
        assert!(restored.contains(&DisplayItem::CpuUsage));
        assert!(!restored.contains(&DisplayItem::MemoryUsage));
    }

    #[test]
    fn test_common_display_item() {
        let builtin = CommonDisplayItem::BuiltIn(DisplayItem::CpuUsage);
        assert!(!builtin.is_plugin());
        assert_eq!(builtin.name(), "CPU:");

        let plugin = CommonDisplayItem::Plugin {
            id: "custom_plugin".to_string(),
            name: "Custom:".to_string(),
        };
        assert!(plugin.is_plugin());
    }
}
