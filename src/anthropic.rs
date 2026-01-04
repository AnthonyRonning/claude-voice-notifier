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

                format!("You are a voice notification assistant. Generate a human-readable summary for voice notification.

                {notification_context}

CRITICAL RULES:
1. Output EXACTLY 1-2 sentences maximum. NO MORE.
2. Must start with 'Claude Code'
3. Use plain English only - absolutely NO code, NO technical syntax, NO file paths, NO command lines
4. Make it conversational and natural for speech
5. Focus on WHAT is happening, not HOW

Examples:
- 'Claude Code needs your permission to install project dependencies.'
- 'Claude Code is waiting for you to approve running a database command.'
- 'Claude Code has a question about the authentication feature.'

OUTPUT ONLY THE SUMMARY. NO EXPLANATIONS. NO CODE.")
            }
            _ => {
                // Default Stop event prompt
                "You are a voice notification assistant. Generate a human-readable summary of what was accomplished.

CRITICAL RULES:
1. Output EXACTLY 1-2 sentences maximum. NO MORE.
2. Must start with 'Claude Code'
3. Use plain English only - absolutely NO code, NO technical syntax, NO file paths, NO variable names
4. Make it conversational and natural for speech
5. Focus on WHAT was done in simple terms, not HOW

Examples:
- 'Claude Code fixed the login bug and added better error handling. The authentication system is now working properly.'
- 'Claude Code implemented the new search feature you requested.'
- 'Claude Code updated the database configuration to improve performance.'
- 'Claude Code has questions about the requirements for the payment system.'

OUTPUT ONLY THE SUMMARY. NO EXPLANATIONS. NO CODE.".to_string()
            }
        };

        let request = AnthropicRequest {
            model: "claude-sonnet-4-20250514".to_string(),
            max_tokens: 100,
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
