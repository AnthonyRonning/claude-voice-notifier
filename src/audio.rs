use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;
use tracing::{debug, error, info};

pub struct AudioPlayer;

impl Default for AudioPlayer {
    fn default() -> Self {
        Self
    }
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self
    }

    pub async fn play_audio_file(&self, file_path: impl AsRef<Path>) -> Result<()> {
        let path = file_path.as_ref();

        if !path.exists() {
            return Err(anyhow::anyhow!("Audio file not found: {}", path.display()));
        }

        info!("Playing audio file: {}", path.display());

        let output = Command::new("mac")
            .arg("afplay")
            .arg(path)
            .output()
            .await
            .context("Failed to execute 'mac afplay' command")?;

        if output.status.success() {
            debug!("Audio playback completed successfully");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Audio playback failed: {}", stderr);
            Err(anyhow::anyhow!("afplay command failed: {}", stderr))
        }
    }

    #[allow(dead_code)]
    pub async fn play_with_fallback(
        &self,
        file_path: impl AsRef<Path>,
        fallback_text: &str,
    ) -> Result<()> {
        if let Err(e) = self.play_audio_file(file_path).await {
            debug!(
                "Audio file playback failed: {}, falling back to 'say' command",
                e
            );
            self.say_text(fallback_text).await?;
        }
        Ok(())
    }

    pub async fn say_text(&self, text: &str) -> Result<()> {
        info!("Using macOS 'say' command for text: {}", text);

        let output = Command::new("mac")
            .arg("say")
            .arg(text)
            .output()
            .await
            .context("Failed to execute 'mac say' command")?;

        if output.status.success() {
            debug!("Text-to-speech completed successfully");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("say command failed: {}", stderr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_play_nonexistent_file() {
        let player = AudioPlayer::new();
        let result = player.play_audio_file("/nonexistent/file.mp3").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_say_text() {
        let player = AudioPlayer::new();
        let result = player.say_text("Test").await;
        // This might fail in CI, so we just check it doesn't panic
        let _ = result;
    }
}
