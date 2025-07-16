use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::{debug, info};

#[derive(Debug, Deserialize)]
struct TranscriptLine {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    line_type: String,
    message: Option<Message>,
}

#[derive(Debug, Deserialize)]
struct Message {
    role: String,
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

pub fn extract_last_assistant_message(transcript_path: &Path) -> Result<String> {
    info!("Reading JSONL transcript from: {:?}", transcript_path);

    let file = fs::File::open(transcript_path)
        .with_context(|| format!("Failed to open transcript file: {transcript_path:?}"))?;
    let reader = BufReader::new(file);

    let mut last_assistant_message: Option<String> = None;
    let mut line_count = 0;

    for line in reader.lines() {
        line_count += 1;
        let line = line.context("Failed to read line")?;

        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Try to parse the line as JSON
        if let Ok(transcript_line) = serde_json::from_str::<TranscriptLine>(&line) {
            // Check if this is an assistant message
            if let Some(message) = transcript_line.message {
                if message.role == "assistant" {
                    // Extract text content
                    if let Some(text_content) = message.content.iter().find_map(|c| {
                        if c.content_type == "text" {
                            c.text.as_ref()
                        } else {
                            None
                        }
                    }) {
                        last_assistant_message = Some(text_content.clone());
                    }
                }
            }
        }
    }

    debug!("Processed {} lines from transcript", line_count);

    match last_assistant_message {
        Some(message) => {
            info!(
                "Found last assistant message with {} characters",
                message.len()
            );
            debug!(
                "Last assistant message preview: {}...",
                &message.chars().take(100).collect::<String>()
            );
            Ok(message)
        }
        None => Err(anyhow::anyhow!("No assistant message found in transcript")),
    }
}
