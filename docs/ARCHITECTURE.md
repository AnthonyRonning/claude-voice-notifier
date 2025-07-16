# Voice Notifier Architecture

## System Overview

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Claude Code    │────▶│  Voice Notifier  │────▶│  Audio Output   │
│     Hook        │     │   (Rust CLI)     │     │  (mac afplay)   │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │
                               ▼
                        ┌──────────────────┐
                        │  ElevenLabs API  │
                        │  (or mac say)    │
                        └──────────────────┘
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

## Data Flow

1. **Input**: Message text from Claude Code hook
2. **Processing**: 
   - Load configuration
   - Generate audio via TTS
   - Save to temporary file
3. **Output**: Play audio notification

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
│   ├── tts.rs
│   ├── audio.rs
│   ├── config.rs
│   └── lib.rs
└── tests/
    └── integration_tests.rs
```