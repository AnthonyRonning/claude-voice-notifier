use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub eleven_labs_api_key: Option<String>,

    #[serde(default = "default_voice_id")]
    pub eleven_labs_voice_id: String,

    #[serde(default = "default_cache_dir")]
    pub cache_dir: PathBuf,

    #[serde(default = "default_model_id")]
    pub eleven_labs_model_id: String,

    #[serde(default)]
    pub anthropic_api_key: Option<String>,
}

fn default_voice_id() -> String {
    // Default voice ID - can be overridden via env
    "21m00Tcm4TlvDq8ikWAM".to_string() // Rachel voice
}

fn default_cache_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("voice-notifier")
        .join("cache")
}

fn default_model_id() -> String {
    "eleven_multilingual_v2".to_string()
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok(); // It's ok if .env doesn't exist

        let config = Config {
            eleven_labs_api_key: std::env::var("ELEVEN_LABS_API_KEY").ok(),
            eleven_labs_voice_id: std::env::var("ELEVEN_LABS_VOICE_ID")
                .unwrap_or_else(|_| default_voice_id()),
            cache_dir: std::env::var("CACHE_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| default_cache_dir()),
            eleven_labs_model_id: std::env::var("ELEVEN_LABS_MODEL_ID")
                .unwrap_or_else(|_| default_model_id()),
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY").ok(),
        };

        Ok(config)
    }

    pub fn ensure_cache_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.cache_dir).context("Failed to create cache directory")?;
        Ok(())
    }

    pub fn has_eleven_labs_config(&self) -> bool {
        self.eleven_labs_api_key.is_some()
    }
}
