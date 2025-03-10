// Settings
//
// This file defines the application settings.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Tick rate in milliseconds
    pub tick_rate: u64,
    /// Default log level filter
    pub default_log_level: Option<String>,
    /// Default application ID filter
    pub default_app_id: Option<String>,
    /// Default context ID filter
    pub default_context_id: Option<String>,
    /// Recent files
    pub recent_files: Vec<PathBuf>,
    /// Maximum number of recent files
    pub max_recent_files: usize,
    /// Theme name
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            tick_rate: 250,
            default_log_level: None,
            default_app_id: None,
            default_context_id: None,
            recent_files: Vec::new(),
            max_recent_files: 10,
            theme: "default".to_string(),
        }
    }
}

impl Settings {
    /// Load settings from a file
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        let settings =
            toml::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(settings)
    }

    /// Save settings to a file
    pub fn save(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, content)
    }

    /// Add a file to the recent files list
    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove the file if it already exists
        self.recent_files.retain(|p| p != &path);

        // Add the file to the beginning of the list
        self.recent_files.insert(0, path);

        // Truncate the list if it's too long
        if self.recent_files.len() > self.max_recent_files {
            self.recent_files.truncate(self.max_recent_files);
        }
    }

    /// Get the default config path
    pub fn default_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("dltui");
        fs::create_dir_all(&path).ok();
        path.push("config.toml");
        path
    }

    /// Load settings from the default path
    pub fn load_default() -> Self {
        let path = Self::default_path();
        Self::load(path).unwrap_or_default()
    }

    /// Save settings to the default path
    pub fn save_default(&self) -> io::Result<()> {
        let path = Self::default_path();
        self.save(path)
    }
}
