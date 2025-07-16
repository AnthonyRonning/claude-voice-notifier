use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    system: String,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
struct Content {
    text: String,
}

pub struct AnthropicClient {
    client: Client,
    api_key: String,
}

impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn summarize(&self, text: &str) -> Result<String> {
        self.summarize_with_context(text, "Stop", None).await
    }

    pub async fn summarize_with_context(
        &self,
        text: &str,
        event_type: &str,
        message: Option<&str>,
    ) -> Result<String> {
        info!(
            "Summarizing text with Anthropic API for event type: {}",
            event_type
        );
        debug!("Text to summarize: {}", text);

        let system_prompt = match event_type {
            "Notification" => {
                let notification_context = if let Some(msg) = message {
                    format!("\n\nNotification context: {}", msg)
                } else {
                    String::new()
                };

                format!("You are a voice notification assistant for Claude Code, an AI coding assistant. Claude Code needs the user's attention.{}

Your summary should:
- ALWAYS start with 'Claude Code' as the subject
- Be exactly one sentence
- Explain what Claude was working on when it needed attention
- If it's waiting for permission, mention what command it wants to run
- If it's been idle, mention what it was last working on
- Be natural when spoken aloud

Examples:
- 'Claude Code needs your permission to run npm install for the React project dependencies.'
- 'Claude Code has been idle for 60 seconds while implementing the authentication module and is waiting for your next instruction.'
- 'Claude Code requires your approval to execute the database migration script.'

Do not include any preamble, explanation, or quotes - just the summary sentence starting with 'Claude Code'.", notification_context)
            }
            _ => {
                // Default Stop event prompt
                "You are a voice notification assistant for Claude Code, an AI coding assistant. When Claude Code finishes helping with a task, you summarize what was accomplished in a clear, informative sentence that will be read aloud as a voice notification.

Your summary should:
- ALWAYS start with 'Claude Code' as the subject
- Be exactly one sentence (can be compound with commas if needed)
- Focus on what Claude actually DID or COMPLETED (not what it said it would do)
- Include specific details like: files created/modified, features implemented, bugs fixed, configurations changed
- Use past tense to indicate completion
- Be natural when spoken aloud
- If Claude asked questions or needs clarification, summarize that instead (e.g., 'Claude Code has questions about...')

Examples of good summaries:
- 'Claude Code successfully implemented the Anthropic API client module, created a transcript parser for JSONL files, and integrated text-to-speech notifications with ElevenLabs.'
- 'Claude Code fixed the authentication bug in login.tsx by updating the JWT token validation and added proper error handling.'
- 'Claude Code refactored the database queries to use prepared statements and added indexes to improve performance by 40 percent.'
- 'Claude Code has a few questions about the notification preferences you'd like for the voice assistant feature.'
- 'Claude Code encountered an error while running tests and needs your help to resolve the failing authentication module.'

Do not include any preamble, explanation, or quotes - just the summary sentence starting with 'Claude Code'.".to_string()
            }
        };

        let request = AnthropicRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 1000,
            messages: vec![Message {
                role: "user".to_string(),
                content: text.to_string(),
            }],
            system: system_prompt,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            error!("Anthropic API error: {} - {}", status, error_text);
            return Err(anyhow!("Anthropic API error: {} - {}", status, error_text));
        }

        let anthropic_response: AnthropicResponse = response.json().await?;

        if let Some(content) = anthropic_response.content.first() {
            let summary = content.text.trim().to_string();
            info!("Generated summary: {}", summary);
            Ok(summary)
        } else {
            Err(anyhow!("No content in Anthropic response"))
        }
    }
}
