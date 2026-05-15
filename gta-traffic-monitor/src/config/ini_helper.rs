//! INI file helper for reading and writing configuration.
//!
//! Port of the original CIniHelper class.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

/// Represents a parsed INI file with sections and key-value pairs.
#[derive(Debug, Clone)]
pub struct IniHelper {
    sections: BTreeMap<String, BTreeMap<String, String>>,
    file_path: Option<String>,
    save_as_utf8: bool,
}

impl IniHelper {
    /// Create a new empty INI helper.
    pub fn new() -> Self {
        Self {
            sections: BTreeMap::new(),
            file_path: None,
            save_as_utf8: true,
        }
    }

    /// Load INI data from a file path.
    pub fn from_file(path: &str) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut helper = Self::from_string(&content);
        helper.file_path = Some(path.to_string());
        Ok(helper)
    }

    /// Parse INI data from a string.
    pub fn from_string(content: &str) -> Self {
        let mut helper = Self::new();
        // Strip UTF-8 BOM if present
        let content = content.strip_prefix('\u{feff}').unwrap_or(content);
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].to_string();
                helper
                    .sections
                    .entry(current_section.clone())
                    .or_default();
            } else if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = Self::unescape(value.trim());
                helper
                    .sections
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, value);
            }
        }
        helper
    }

    /// Get a string value.
    pub fn get_string(&self, section: &str, key: &str, default: &str) -> String {
        self.sections
            .get(section)
            .and_then(|s| s.get(key))
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Set a string value.
    pub fn set_string(&mut self, section: &str, key: &str, value: &str) {
        self.sections
            .entry(section.to_string())
            .or_default()
            .insert(key.to_string(), value.to_string());
    }

    /// Get an integer value.
    pub fn get_int(&self, section: &str, key: &str, default: i64) -> i64 {
        self.sections
            .get(section)
            .and_then(|s| s.get(key))
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Set an integer value.
    pub fn set_int(&mut self, section: &str, key: &str, value: i64) {
        self.set_string(section, key, &value.to_string());
    }

    /// Get a boolean value.
    pub fn get_bool(&self, section: &str, key: &str, default: bool) -> bool {
        self.get_int(section, key, default as i64) != 0
    }

    /// Set a boolean value.
    pub fn set_bool(&mut self, section: &str, key: &str, value: bool) {
        self.set_int(section, key, value as i64);
    }

    /// Get a float value.
    pub fn get_float(&self, section: &str, key: &str, default: f64) -> f64 {
        self.sections
            .get(section)
            .and_then(|s| s.get(key))
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Set a float value.
    pub fn set_float(&mut self, section: &str, key: &str, value: f64) {
        self.set_string(section, key, &value.to_string());
    }

    /// Get all section names.
    pub fn section_names(&self) -> Vec<String> {
        self.sections.keys().cloned().collect()
    }

    /// Get all section names with a given prefix.
    pub fn section_names_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.sections
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// Get all key-value pairs in a section.
    pub fn get_section(&self, section: &str) -> Option<&BTreeMap<String, String>> {
        self.sections.get(section)
    }

    /// Remove a section.
    pub fn remove_section(&mut self, section: &str) -> bool {
        self.sections.remove(section).is_some()
    }

    /// Save the INI data to the file it was loaded from, or the specified path.
    pub fn save(&self) -> io::Result<()> {
        let path = self
            .file_path
            .as_deref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No file path set"))?;
        self.save_to(path)
    }

    /// Save the INI data to a specific path.
    pub fn save_to(&self, path: &str) -> io::Result<()> {
        let content = self.to_string();
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        if self.save_as_utf8 {
            // Write UTF-8 BOM + content
            let mut data = vec![0xEF, 0xBB, 0xBF];
            data.extend_from_slice(content.as_bytes());
            fs::write(path, data)
        } else {
            fs::write(path, content)
        }
    }

    /// Set whether to save as UTF-8 with BOM.
    pub fn set_save_as_utf8(&mut self, utf8: bool) {
        self.save_as_utf8 = utf8;
    }

    /// Serialize to INI format string.
    fn to_string(&self) -> String {
        let mut output = String::new();
        for (section, entries) in &self.sections {
            output.push_str(&format!("[{}]\n", section));
            for (key, value) in entries {
                output.push_str(&format!("{}={}\n", key, Self::escape(value)));
            }
            output.push('\n');
        }
        output
    }

    /// Escape special characters in INI values.
    fn escape(value: &str) -> String {
        value
            .replace('\\', "\\\\")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    }

    /// Unescape special characters in INI values.
    fn unescape(value: &str) -> String {
        let mut result = String::with_capacity(value.len());
        let mut chars = value.chars();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('\\') => result.push('\\'),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(c);
            }
        }
        result
    }
}

impl Default for IniHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ini_string() {
        let content = "[general]\nname=TrafficMonitor\nversion=1\n\n[display]\nshow=1\n";
        let ini = IniHelper::from_string(content);

        assert_eq!(ini.get_string("general", "name", ""), "TrafficMonitor");
        assert_eq!(ini.get_int("general", "version", 0), 1);
        assert_eq!(ini.get_bool("display", "show", false), true);
    }

    #[test]
    fn test_get_default_values() {
        let ini = IniHelper::new();
        assert_eq!(ini.get_string("x", "y", "default"), "default");
        assert_eq!(ini.get_int("x", "y", 42), 42);
        assert_eq!(ini.get_bool("x", "y", true), true);
    }

    #[test]
    fn test_set_and_get() {
        let mut ini = IniHelper::new();
        ini.set_string("sec", "key", "value");
        ini.set_int("sec", "num", 123);
        ini.set_bool("sec", "flag", true);

        assert_eq!(ini.get_string("sec", "key", ""), "value");
        assert_eq!(ini.get_int("sec", "num", 0), 123);
        assert_eq!(ini.get_bool("sec", "flag", false), true);
    }

    #[test]
    fn test_escape_unescape() {
        let original = "line1\nline2\\end";
        let escaped = IniHelper::escape(original);
        let unescaped = IniHelper::unescape(&escaped);
        assert_eq!(unescaped, original);
    }

    #[test]
    fn test_utf8_bom_stripping() {
        let content = "\u{feff}[section]\nkey=value\n";
        let ini = IniHelper::from_string(content);
        assert_eq!(ini.get_string("section", "key", ""), "value");
    }

    #[test]
    fn test_comments_ignored() {
        let content = "; comment\n# another comment\n[section]\nkey=value\n";
        let ini = IniHelper::from_string(content);
        assert_eq!(ini.get_string("section", "key", ""), "value");
    }

    #[test]
    fn test_section_names() {
        let content = "[alpha]\na=1\n[beta]\nb=2\n[alpha_extra]\nc=3\n";
        let ini = IniHelper::from_string(content);
        assert_eq!(ini.section_names().len(), 3);
        assert_eq!(ini.section_names_with_prefix("alpha").len(), 2);
    }

    #[test]
    fn test_remove_section() {
        let mut ini = IniHelper::from_string("[sec]\nk=v\n");
        assert!(ini.remove_section("sec"));
        assert!(!ini.remove_section("sec"));
        assert_eq!(ini.get_string("sec", "k", "gone"), "gone");
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.ini");
        let path_str = path.to_str().unwrap();

        let mut ini = IniHelper::new();
        ini.set_string("app", "name", "GTA-TrafficMonitor");
        ini.set_int("app", "version", 1);
        ini.save_to(path_str).unwrap();

        let loaded = IniHelper::from_file(path_str).unwrap();
        assert_eq!(loaded.get_string("app", "name", ""), "GTA-TrafficMonitor");
        assert_eq!(loaded.get_int("app", "version", 0), 1);
    }
}
