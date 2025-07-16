# Voice Notifier Master Todo List

## ✅ Completed
- [x] Create project documentation structure
- [x] Set up Rust project with Cargo.toml
- [x] Create Nix flake for development environment

## 🔧 Core Implementation

### 1. Basic Audio Playback Module
- [ ] Create `src/audio.rs` module
- [ ] Implement `play_audio_file()` function using `mac afplay`
- [ ] Handle command execution errors gracefully
- [ ] Add async support with tokio::process
- [ ] Test with provided `claude_has_finished.mp3`
- [ ] Add logging for debugging audio playback issues

### 2. ElevenLabs TTS Integration
- [ ] Create `src/tts.rs` module
- [ ] Define ElevenLabs API client structure
- [ ] Implement authentication with API key from .env
- [ ] Create `generate_speech()` function
  - [ ] Accept text input
  - [ ] Handle voice ID selection
  - [ ] Stream audio response to temporary file
  - [ ] Return file path or error
- [ ] Add retry logic with exponential backoff
- [ ] Implement rate limiting awareness
- [ ] Add comprehensive error types for API failures

### 3. Fallback System
- [ ] Create `src/fallback.rs` module
- [ ] Implement fallback chain logic:
  1. Try ElevenLabs API
  2. Use cached audio if available
  3. Fall back to `mac say` command
- [ ] Cache management:
  - [ ] Create cache directory on first run
  - [ ] Save successful ElevenLabs audio as default.mp3
  - [ ] Verify cache file integrity
  - [ ] Handle missing/corrupted cache files
- [ ] Implement `mac say` wrapper
  - [ ] Text-to-speech using system voice
  - [ ] Handle voice selection if available
  - [ ] Error handling for missing `say` command

### 4. Configuration Module
- [ ] Create `src/config.rs`
- [ ] Load environment variables from .env
- [ ] Define configuration struct with:
  - [ ] ElevenLabs API key
  - [ ] Voice ID (with default)
  - [ ] Cache directory path
  - [ ] Log level
  - [ ] Timeout settings
- [ ] Validate configuration on load
- [ ] Provide sensible defaults
- [ ] Support config file override (future)

### 5. CLI Interface
- [ ] Create main.rs with clap argument parser
- [ ] Define CLI arguments:
  - [ ] `--message` / `-m`: Custom message text
  - [ ] `--voice`: Override voice ID
  - [ ] `--no-cache`: Skip cache, always use API
  - [ ] `--fallback-only`: Test fallback behavior
  - [ ] `--verbose` / `-v`: Enable debug logging
- [ ] Implement main execution flow
- [ ] Add graceful shutdown handling
- [ ] Return appropriate exit codes

### 6. Error Handling & Logging
- [ ] Set up tracing/logging infrastructure
- [ ] Define custom error types with thiserror
- [ ] Implement error chain for debugging
- [ ] Add context to errors (which step failed)
- [ ] Log to stderr by default
- [ ] Optional file logging

### 7. Testing
- [ ] Unit tests for each module
- [ ] Integration tests for fallback chain
- [ ] Mock ElevenLabs API for testing
- [ ] Test error scenarios:
  - [ ] Missing API key
  - [ ] Network failures
  - [ ] Invalid audio files
  - [ ] Missing commands
- [ ] Performance tests for cache hits

### 8. Claude Code Hook Integration
- [ ] Create `docs/CLAUDE_HOOKS.md`
- [ ] Document hook configuration format
- [ ] Provide example hook scripts
- [ ] Test with real Claude Code hooks
- [ ] Handle various message formats
- [ ] Parse task completion details

### 9. Advanced Features (Future)
- [ ] Multiple voice profiles
- [ ] Custom message templates
- [ ] Notification history
- [ ] Web UI for configuration
- [ ] Support for other TTS providers
- [ ] Cross-platform support (Linux native audio)

## 📝 Documentation Tasks
- [ ] Create comprehensive README.md
- [ ] Add usage examples
- [ ] Document all CLI options
- [ ] Create troubleshooting guide
- [ ] Add contribution guidelines
- [ ] Generate API documentation

## 🚀 Release Tasks
- [ ] Create GitHub Actions CI/CD
- [ ] Add release workflow
- [ ] Create homebrew formula
- [ ] Package as nix derivation
- [ ] Create demo video/gif
- [ ] Write announcement post