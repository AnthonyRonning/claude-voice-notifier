# Voice Notifier Architecture

## System Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Claude Code    │────▶│  Voice Notifier  │────▶│  Audio Output   │
│     Hook        │     │   (Rust CLI)     │     │  (mac afplay)   │
└─────────────────┘     └──────────────────┘     └─────────────────┘
         │                     │    │
         │                     │    └──────────────┐
         ▼                     ▼                   ▼
┌─────────────────┐     ┌──────────────────┐  ┌─────────────────┐
│   Transcript    │     │  Anthropic API   │  │  ElevenLabs API │
│   (JSONL)       │     │  (Claude 4)      │  │  (or mac say)   │
└─────────────────┘     └──────────────────┘  └─────────────────┘
```

## Component Design

### CLI Interface (`main.rs`)
- Parse command-line arguments
- Load environment configuration
- Orchestrate notification flow

### TTS Module (`tts.rs`)
- ElevenLabs API client
- Fallback to macOS `say`
- Audio file generation

### Audio Module (`audio.rs`)
- Interface to `mac afplay`
- Temporary file management
- Playback control

### Config Module (`config.rs`)
- `.env` file parsing
- Runtime configuration
- Voice profile management
- API key management (ElevenLabs, Anthropic)

### Transcript Module (`transcript.rs`)
- Parse JSONL transcript files from Claude Code
- Extract last assistant message
- Handle different message types and formats

### Anthropic Module (`anthropic.rs`)
- Claude 4 Sonnet API client
- Intelligent summarization of Claude's responses
- Context-aware prompts for different event types
- Fallback to simple truncation on errors

## Data Flow

1. **Input**: Hook event from Claude Code (Stop or Notification)
   - Transcript path (JSONL file)
   - Event type and optional message
2. **Processing**: 
   - Load configuration (API keys, voice settings)
   - Parse transcript to extract last assistant message
   - Summarize message using Anthropic API (Claude 4 Sonnet)
   - Generate audio via ElevenLabs TTS
   - Save to temporary file
3. **Output**: Play audio notification with intelligent summary

## Error Handling Strategy

```rust
enum NotificationError {
    ConfigError(String),
    TtsError(String),
    AudioError(String),
}

// Fallback chain:
// 1. ElevenLabs API (fresh generation)
// 2. Cached "Task complete" audio file
// 3. macOS say command
// 4. Log error (silent failure)
```

## Caching Strategy

### Default Cache
- Pre-generated "Claude has finished a task" audio file
- Stored in `~/.config/voice-notifier/cache/default.mp3`
- Generated on first successful ElevenLabs call
- Used as immediate fallback if API fails

### Cache Structure
```
~/.config/voice-notifier/
├── cache/
│   └── default.mp3  # Pre-generated fallback audio
└── config.toml      # User preferences
```

## File Structure

```
voice-notifier/
├── Cargo.toml
├── flake.nix
├── .env.example
├── docs/
│   ├── PROJECT_GOALS.md
│   ├── ARCHITECTURE.md
│   ├── CLAUDE_HOOKS.md
│   ├── IMPLEMENTATION_NOTES.md
│   └── MASTER_TODO.md
├── src/
│   ├── main.rs
│   ├── anthropic.rs
│   ├── audio.rs
│   ├── config.rs
│   ├── transcript.rs
│   ├── tts.rs
│   └── lib.rs
├── claude_stop_hook.sh
└── tests/
    └── integration_tests.rs
```