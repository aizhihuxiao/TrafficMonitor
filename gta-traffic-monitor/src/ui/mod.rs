//! UI module for the application interface.
//!
//! This module contains the UI abstractions that can be implemented
//! with platform-specific backends.

pub mod app;
pub mod tray;

pub use app::App;
