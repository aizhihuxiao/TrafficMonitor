//! Skin system for customizable UI appearance.
//!
//! Port of the original SkinFile.h/cpp and SkinManager.

pub mod skin_file;
pub mod skin_manager;

pub use skin_file::{Layout, LayoutItem, SkinFile, SkinInfo};
pub use skin_manager::SkinManager;
