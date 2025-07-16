use anyhow::Result;
use clap::Parser;
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

mod audio;
mod config;
mod tts;

use audio::AudioPlayer;
use config::Config;
use tts::ElevenLabsClient;

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

    let text = if args.test {
        info!("Running in test mode");
        "Claude has finished a task".to_string()
    } else if let Some(text) = args.text {
        text
    } else if args.file.is_some() {
        // Just play the file, no TTS needed
        return player.play_audio_file(args.file.unwrap()).await;
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
            match player.play_audio_file(&cache_file).await {
                Ok(_) => return Ok(()),
                Err(e) => error!("Failed to play cached audio: {}", e),
            }
        }
    }

    // Final fallback: mac say
    info!("Using mac say as final fallback");
    player.say_text(&text).await?;

    Ok(())
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
        match player.play_audio_file(&debug_path).await {
            Ok(_) => {}
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
        let result = player.play_audio_file(&temp_path).await;

        // Clean up temp file
        if let Err(e) = tokio::fs::remove_file(&temp_path).await {
            debug!("Failed to remove temp file: {}", e);
        }

        result?;
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
