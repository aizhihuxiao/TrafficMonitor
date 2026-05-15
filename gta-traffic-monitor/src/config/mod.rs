//! Configuration management module.
//!
//! Handles loading and saving application settings, corresponding to
//! the original INI-based configuration system.

pub mod ini_helper;
pub mod settings;

pub use settings::{
    AppConfig, GeneralSettings, MainWndSettings, TaskBarSettings,
};
