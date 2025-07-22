use anyhow::{Context, Result};
use reqwest::Client;
use serde::Serialize;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};

#[derive(Debug, Serialize)]
struct TextToSpeechRequest {
    text: String,
    model_id: String,
    voice_settings: VoiceSettings,
}

#[derive(Debug, Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    style: f32,
    use_speaker_boost: bool,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            stability: 0.5,
            similarity_boost: 0.5,
            style: 0.0,
            use_speaker_boost: true,
        }
    }
}

pub struct ElevenLabsClient {
    client: Client,
    api_key: String,
    voice_id: String,
    model_id: String,
}

impl ElevenLabsClient {
    pub fn new(api_key: String, voice_id: String, model_id: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            voice_id,
            model_id,
        }
    }

    pub async fn generate_speech(&self, text: &str, output_path: &Path) -> Result<()> {
        const MAX_TTS_LENGTH: usize = 1000;
        
        let truncated_text = if text.len() > MAX_TTS_LENGTH {
            info!(
                "Text too long ({} chars), truncating to {} chars",
                text.len(),
                MAX_TTS_LENGTH
            );
            format!("{}...", &text[..MAX_TTS_LENGTH])
        } else {
            text.to_string()
        };
        
        info!("Generating speech with ElevenLabs for text: {}", truncated_text);

        let url = format!(
            "https://api.elevenlabs.io/v1/text-to-speech/{}",
            self.voice_id
        );

        let request_body = TextToSpeechRequest {
            text: truncated_text,
            model_id: self.model_id.clone(),
            voice_settings: VoiceSettings::default(),
        };

        debug!("Sending request to ElevenLabs API");
        let response = self
            .client
            .post(&url)
            .header("Accept", "audio/mpeg")
            .header("Content-Type", "application/json")
            .header("xi-api-key", &self.api_key)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to ElevenLabs")?;

        debug!("Response headers: {:?}", response.headers());

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!(
                "ElevenLabs API error ({}): {}",
                status,
                error_text
            ));
        }

        debug!("Downloading audio data");
        let audio_data = response
            .bytes()
            .await
            .context("Failed to download audio data")?;

        debug!("Writing audio to file: {}", output_path.display());
        let mut file = File::create(output_path)
            .await
            .context("Failed to create output file")?;

        file.write_all(&audio_data)
            .await
            .context("Failed to write audio data")?;

        file.flush().await.context("Failed to flush file")?;
        drop(file); // Ensure file is closed

        // Get file size for debugging
        let metadata = tokio::fs::metadata(output_path).await?;
        info!(
            "Successfully generated speech file: {} (size: {} bytes)",
            output_path.display(),
            metadata.len()
        );

        // Debug: Check first few bytes to verify it's an MP3
        let mut debug_file = tokio::fs::File::open(output_path).await?;
        let mut header = vec![0u8; 4];
        use tokio::io::AsyncReadExt;
        debug_file.read_exact(&mut header).await?;
        debug!("File header bytes: {:?} (hex: {:02x?})", header, header);

        Ok(())
    }
}
