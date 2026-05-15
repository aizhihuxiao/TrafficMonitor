//! Skin file parser and renderer.
//!
//! Port of the original CSkinFile class.

use crate::config::settings::Color;
use crate::config::settings::FontInfo;
use crate::display::items::DisplayItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Text alignment within a display item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Right,
    Center,
    /// Both sides (label left, value right)
    Side,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

/// Layout information for a single display item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutItem {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Width of the item
    pub width: i32,
    /// Text alignment
    pub align: Alignment,
    /// Whether this item is visible
    pub show: bool,
    /// Custom label text (if any)
    pub label: Option<String>,
}

impl Default for LayoutItem {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 100,
            align: Alignment::Left,
            show: true,
            label: None,
        }
    }
}

/// Layout configuration for the skin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
    /// Total width of the skin
    pub width: i32,
    /// Total height of the skin
    pub height: i32,
    /// Layout items for each display item
    pub items: HashMap<String, LayoutItem>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            width: 280,
            height: 43,
            items: HashMap::new(),
        }
    }
}

/// Skin metadata and color information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInfo {
    /// Text colors for each display item
    pub text_colors: Vec<Color>,
    /// Whether each item has its own color
    pub specify_each_item_color: bool,
    /// Skin author
    pub author: String,
    /// Font information
    pub font: FontInfo,
    /// Display text labels
    pub display_text: HashMap<String, String>,
}

impl Default for SkinInfo {
    fn default() -> Self {
        Self {
            text_colors: vec![Color::new(0, 255, 0)],
            specify_each_item_color: false,
            author: String::new(),
            font: FontInfo::default(),
            display_text: HashMap::new(),
        }
    }
}

/// A complete skin definition.
///
/// Port of the original CSkinFile class.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinFile {
    /// Skin name
    pub name: String,
    /// Skin metadata
    pub info: SkinInfo,
    /// Normal layout
    pub layout_small: Layout,
    /// Extended layout (show more info)
    pub layout_large: Layout,
    /// Whether the skin uses PNG background
    pub is_png: bool,
    /// Display items defined in this skin
    pub display_items: Vec<DisplayItem>,
}

impl SkinFile {
    /// Create a new default skin.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            info: SkinInfo::default(),
            layout_small: Layout::default(),
            layout_large: Layout {
                width: 280,
                height: 57,
                ..Layout::default()
            },
            is_png: false,
            display_items: vec![
                DisplayItem::Upload,
                DisplayItem::Download,
                DisplayItem::CpuUsage,
                DisplayItem::MemoryUsage,
            ],
        }
    }

    /// Load a skin from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let skin: SkinFile = serde_json::from_str(&content)?;
        Ok(skin)
    }

    /// Save the skin to a JSON file.
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get display items in this skin.
    pub fn display_items(&self) -> &[DisplayItem] {
        &self.display_items
    }
}

impl Default for SkinFile {
    fn default() -> Self {
        Self::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_skin() {
        let skin = SkinFile::default();
        assert_eq!(skin.name, "default");
        assert!(!skin.display_items.is_empty());
        assert_eq!(skin.layout_small.width, 280);
    }

    #[test]
    fn test_skin_serialization() {
        let skin = SkinFile::new("test_skin");
        let json = serde_json::to_string(&skin).unwrap();
        let deserialized: SkinFile = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test_skin");
    }

    #[test]
    fn test_skin_save_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("skin.json");
        let path_str = path.to_str().unwrap();

        let skin = SkinFile::new("my_skin");
        skin.save_to_file(path_str).unwrap();

        let loaded = SkinFile::load_from_file(path_str).unwrap();
        assert_eq!(loaded.name, "my_skin");
        assert_eq!(loaded.display_items.len(), skin.display_items.len());
    }
}
