use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::time::{SystemTime, Duration};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

mod anthropic;
mod audio;
mod config;
mod transcript;
mod tts;

use anthropic::AnthropicClient;
use audio::AudioPlayer;
use config::Config;
use transcript::extract_last_assistant_message;
use tts::ElevenLabsClient;

// Simple RAII lock guard that removes the lock file when dropped
struct LockGuard {
    path: PathBuf,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Audio file to play")]
    file: Option<String>,

    #[arg(short = 's', long, help = "Text to speak")]
    text: Option<String>,

    #[arg(short, long, help = "Test mode: play default notification")]
    test: bool,

    #[arg(long, help = "Force use of mac say (skip ElevenLabs)")]
    force_say: bool,

    #[arg(long, help = "Keep temporary files for debugging")]
    keep_temp: bool,

    #[arg(long, help = "Transcript file path from Claude")]
    transcript: Option<PathBuf>,

    #[arg(long, help = "Hook event type (Stop or Notification)")]
    hook_event: Option<String>,

    #[arg(long, help = "Hook message (for Notification events)")]
    hook_message: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("voice_notifier=info")),
        )
        .init();

    let args = Args::parse();
    let config = Config::from_env()?;
    let player = AudioPlayer::new();

    // Ensure cache directory exists
    if let Err(e) = config.ensure_cache_dir() {
        error!("Failed to create cache directory: {}", e);
    }

    // Check for active notification lock
    let lock_file = config.cache_dir.join("notification.lock");
    debug!("Checking for lock file at: {:?}", lock_file);
    if lock_file.exists() {
        debug!("Lock file exists, checking age");
        // Check if lock is stale (older than 30 seconds)
        if let Ok(metadata) = fs::metadata(&lock_file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                    if elapsed < Duration::from_secs(30) {
                        info!("Another notification is in progress (lock age: {:?}), skipping", elapsed);
                        return Ok(());
                    } else {
                        debug!("Removing stale lock file (age: {:?})", elapsed);
                        let _ = fs::remove_file(&lock_file);
                    }
                }
            }
        }
    } else {
        debug!("No lock file found");
    }

    // Create lock file
    if let Err(e) = fs::write(&lock_file, std::process::id().to_string()) {
        error!("Failed to create lock file: {}", e);
        // Continue anyway, as this shouldn't block notifications entirely
    }

    // Ensure lock file is removed on exit
    let _lock_guard = LockGuard { path: lock_file };

    // Check if system is muted before processing text notifications
    if player.is_system_muted().await {
        info!("System is muted, skipping voice notification to avoid charges");
        return Ok(());
    }

    let text = if args.test {
        info!("Running in test mode");
        "Claude has finished a task".to_string()
    } else if let Some(text) = args.text {
        text
    } else if args.file.is_some() {
        // Just play the file, no TTS needed
        return player.play_audio_file_background(args.file.unwrap()).await;
    } else if let Some(transcript_path) = args.transcript {
        // Process transcript to get summary
        if let Some(event_type) = &args.hook_event {
            match process_transcript_with_context(
                &config,
                &transcript_path,
                event_type,
                args.hook_message.as_deref(),
            )
            .await
            {
                Ok(summary) => summary,
                Err(e) => {
                    error!("Failed to process transcript: {}", e);
                    match event_type.as_str() {
                        "Notification" => "Claude Code needs your attention".to_string(),
                        _ => "Claude has finished a task".to_string(),
                    }
                }
            }
        } else {
            // Legacy mode without event type
            match process_transcript(&config, &transcript_path).await {
                Ok(summary) => summary,
                Err(e) => {
                    error!("Failed to process transcript: {}", e);
                    "Claude has finished a task".to_string()
                }
            }
        }
    } else {
        "Claude has finished a task".to_string()
    };

    // Try different methods in order
    if !args.force_say && config.has_eleven_labs_config() {
        match generate_and_play_elevenlabs(&config, &player, &text, args.keep_temp).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                error!("ElevenLabs failed: {}", e);
                info!("Falling back to alternative methods");
            }
        }
    }

    // Try cached audio if it's the default message
    if text == "Claude has finished a task" {
        let cache_file = config.cache_dir.join("default.mp3");
        if cache_file.exists() {
            info!("Using cached audio file");
            match player.play_audio_file_background(&cache_file).await {
                Ok(_) => return Ok(()),
                Err(e) => error!("Failed to play cached audio: {}", e),
            }
        }
    }

    // Final fallback: mac say
    info!("Using mac say as final fallback");
    player.say_text_background(&text).await?;

    Ok(())
}

async fn process_transcript(config: &Config, transcript_path: &PathBuf) -> Result<String> {
    info!("Processing transcript from: {:?}", transcript_path);

    // Extract the last assistant message
    let last_message = extract_last_assistant_message(transcript_path)?;

    // If we have an Anthropic API key, summarize the message
    if let Some(api_key) = &config.anthropic_api_key {
        let client = AnthropicClient::new(api_key.clone());
        match client.summarize(&last_message).await {
            Ok(summary) => {
                info!("Successfully generated summary");
                Ok(summary)
            }
            Err(e) => {
                error!("Failed to summarize with Anthropic: {}", e);
                // Fallback to a simple truncation
                Ok(truncate_message(&last_message))
            }
        }
    } else {
        info!("No Anthropic API key configured, using simple truncation");
        Ok(truncate_message(&last_message))
    }
}

async fn process_transcript_with_context(
    config: &Config,
    transcript_path: &PathBuf,
    event_type: &str,
    message: Option<&str>,
) -> Result<String> {
    info!(
        "Processing transcript from: {:?} for event: {}",
        transcript_path, event_type
    );

    // Extract the last assistant message
    let last_message = extract_last_assistant_message(transcript_path)?;

    // If we have an Anthropic API key, summarize the message with context
    if let Some(api_key) = &config.anthropic_api_key {
        let client = AnthropicClient::new(api_key.clone());
        match client
            .summarize_with_context(&last_message, event_type, message)
            .await
        {
            Ok(summary) => {
                info!("Successfully generated summary");
                Ok(summary)
            }
            Err(e) => {
                error!("Failed to summarize with Anthropic: {}", e);
                // Fallback based on event type
                match event_type {
                    "Notification" => Ok("Claude Code needs your attention".to_string()),
                    _ => Ok(truncate_message(&last_message)),
                }
            }
        }
    } else {
        info!("No Anthropic API key configured, using simple message");
        match event_type {
            "Notification" => Ok("Claude Code needs your attention".to_string()),
            _ => Ok(truncate_message(&last_message)),
        }
    }
}

fn truncate_message(message: &str) -> String {
    // Simple fallback: take first sentence or first 100 chars
    let trimmed = message.trim();

    // Try to find first sentence
    if let Some(end) = trimmed.find(['.', '!', '?']) {
        let sentence = &trimmed[..=end];
        if sentence.len() <= 150 {
            return sentence.to_string();
        }
    }

    // Otherwise, truncate at word boundary
    if trimmed.len() <= 100 {
        trimmed.to_string()
    } else {
        let truncated = &trimmed[..100];
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}...", &truncated[..last_space])
        } else {
            format!("{truncated}...")
        }
    }
}

async fn generate_and_play_elevenlabs(
    config: &Config,
    player: &AudioPlayer,
    text: &str,
    keep_temp: bool,
) -> Result<()> {
    let api_key = config
        .eleven_labs_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("ElevenLabs API key not configured"))?;

    let client = ElevenLabsClient::new(
        api_key.clone(),
        config.eleven_labs_voice_id.clone(),
        config.eleven_labs_model_id.clone(),
    );

    let temp_path = if keep_temp {
        let debug_path = std::env::current_dir()?.join("debug_audio.mp3");
        info!("Saving debug audio to: {}", debug_path.display());
        client.generate_speech(text, &debug_path).await?;
        match player.play_audio_file_background(&debug_path).await {
            Ok(_) => {
                info!("Audio playing in background, debug file saved at: {}", debug_path.display());
            }
            Err(e) => {
                error!("Failed to play audio: {}", e);
                info!("Debug file saved at: {}", debug_path.display());
                return Err(e);
            }
        }
        debug_path
    } else {
        // Use cache directory for temp files to avoid permission issues
        config.ensure_cache_dir()?;
        let temp_path = config
            .cache_dir
            .join(format!("temp_voice_notifier_{}.mp3", std::process::id()));

        client.generate_speech(text, &temp_path).await?;
        
        // Play audio in background - don't wait for completion
        player.play_audio_file_background(&temp_path).await?;
        
        // Don't clean up temp file immediately since audio is playing in background
        // The OS will clean it up eventually from the temp directory
        temp_path
    };

    // If this is the default message, cache it
    if text == "Claude has finished a task" {
        let cache_file = config.cache_dir.join("default.mp3");
        if let Err(e) = tokio::fs::copy(temp_path, &cache_file).await {
            error!("Failed to cache audio file: {}", e);
        } else {
            info!("Cached default audio for future use");
        }
    }

    Ok(())
}
