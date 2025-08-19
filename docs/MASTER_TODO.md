# Voice Notifier Master Todo List

## ✅ Completed
- [x] Create project documentation structure
- [x] Set up Rust project with Cargo.toml
- [x] Create Nix flake for development environment
- [x] Basic Audio Playback Module (audio.rs)
- [x] ElevenLabs TTS Integration (tts.rs)
- [x] Configuration Module (config.rs)
- [x] CLI Interface with clap
- [x] Error handling and logging with tracing
- [x] Fallback system (ElevenLabs → cached → mac say)
- [x] Claude Code Stop hook integration
- [x] Fixed nix-shell temp directory issues (documented in IMPLEMENTATION_NOTES.md)
- [x] Create comprehensive README.md
- [x] Parse task completion details from transcript
- [x] Anthropic API Integration (anthropic.rs)
- [x] Transcript parsing module (transcript.rs)
- [x] Support for both Stop and Notification hooks
- [x] Context-aware summaries based on event type

## 🔧 Core Implementation

### 1. Basic Audio Playback Module ✅ COMPLETED
- [x] Create `src/audio.rs` module
- [x] Implement `play_audio_file()` function using `mac afplay`
- [x] Handle command execution errors gracefully
- [x] Add async support with tokio::process
- [x] Test with provided `claude_has_finished.mp3`
- [x] Add logging for debugging audio playback issues

### 2. ElevenLabs TTS Integration ✅ COMPLETED
- [x] Create `src/tts.rs` module
- [x] Define ElevenLabs API client structure
- [x] Implement authentication with API key from .env
- [x] Create `generate_speech()` function
  - [x] Accept text input
  - [x] Handle voice ID selection
  - [x] Stream audio response to temporary file
  - [x] Return file path or error
- [x] Add comprehensive error types for API failures
- [ ] Add retry logic with exponential backoff
- [ ] Implement rate limiting awareness

### 3. Fallback System ✅ COMPLETED
- [x] Implement fallback chain logic (in main.rs):
  1. Try ElevenLabs API
  2. Use cached audio if available
  3. Fall back to `mac say` command
- [x] Cache management:
  - [x] Create cache directory on first run
  - [x] Save successful ElevenLabs audio as default.mp3
  - [x] Handle missing/corrupted cache files
- [x] Implement `mac say` wrapper (in audio.rs)
  - [x] Text-to-speech using system voice
  - [x] Error handling for missing `say` command
- [ ] Verify cache file integrity

### 4. Configuration Module ✅ COMPLETED
- [x] Create `src/config.rs`
- [x] Load environment variables from .env
- [x] Define configuration struct with:
  - [x] ElevenLabs API key
  - [x] Voice ID (with default)
  - [x] Cache directory path
  - [x] Model ID configuration
- [x] Provide sensible defaults
- [ ] Log level configuration
- [ ] Timeout settings
- [ ] Validate configuration on load
- [ ] Support config file override (future)

### 5. CLI Interface ✅ COMPLETED
- [x] Create main.rs with clap argument parser
- [x] Define CLI arguments:
  - [x] `--text` / `-s`: Custom message text
  - [x] `--file` / `-f`: Play audio file
  - [x] `--test` / `-t`: Test mode
  - [x] `--force-say`: Force fallback behavior
  - [x] `--keep-temp`: Keep temp files for debugging
- [x] Implement main execution flow
- [x] Return appropriate exit codes
- [ ] `--voice`: Override voice ID
- [ ] `--verbose` / `-v`: Enable debug logging
- [ ] Add graceful shutdown handling

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

### 8. Claude Code Hook Integration ✅ COMPLETED
- [x] Create `docs/CLAUDE_HOOKS.md`
- [x] Document hook configuration format
- [x] Create Stop hook script (claude_stop_hook.sh)
- [x] Test with real Claude Code hooks
- [x] Handle Stop hook JSON format
- [x] Return proper approval decision
- [ ] Parse task completion details from transcript

## ✅ Phase 2: Intelligent Summaries (COMPLETED)

### 1. Transcript Parsing ✅ COMPLETED
- [x] Parse JSONL transcript file from Stop hook
- [x] Extract last assistant message content
- [x] Handle malformed JSON gracefully
- [x] Support for reading line-by-line JSONL format

### 2. Anthropic API Integration ✅ COMPLETED
- [x] Add Anthropic API client (custom implementation)
- [x] Create summarizer module (anthropic.rs)
- [x] Implement API authentication with Claude 4 Sonnet
- [x] Create prompt for one-sentence summaries
- [x] Handle API errors with fallback to truncation

### 3. Enhanced Hook Processing ✅ COMPLETED
- [x] Support both Stop and Notification hooks
- [x] Extract and summarize Claude's responses
- [x] Generate contextual voice notifications
- [x] Pass hook event type and message to binary
- [x] Context-aware prompts for different event types
- [ ] Cache summaries to avoid duplicate API calls

### 9. Advanced Features (Future)
- [ ] Multiple voice profiles
- [ ] Custom message templates
- [ ] Notification history
- [ ] Web UI for configuration
- [ ] Support for other TTS providers
- [ ] Cross-platform support (Linux native audio)

### 10. Concurrent Notification Handling (Future Enhancement)
**Alternative approaches for preventing simultaneous voice notifications:**

#### Queue System Approach:
- Implement a notification daemon that manages a queue
- Each notification request adds to the queue instead of playing immediately
- Daemon processes queue sequentially, ensuring only one plays at a time
- Benefits: No missed notifications, controlled playback order
- Drawbacks: More complex architecture, requires daemon process

#### Cooldown Period Approach:
- Implement a time-based cooldown after each notification
- Ignore or defer notifications within X seconds of the last one
- Could be configurable (e.g., 2-5 second cooldown)
- Benefits: Simple to implement, prevents rapid-fire notifications
- Drawbacks: Might miss important notifications during cooldown

### 11. Audio Ducking / Music App Control (Future Enhancement)
**Research completed on music app control during playback:**

#### Findings:
- System volume ducking affects ALL audio (including the notification) - not viable
- Apple Music supports AppleScript volume control: `set sound volume to 20`
- Spotify supports AppleScript volume control (when running)
- Amperfy only supports pause/play via UI automation, not volume control

#### Implementation Options:
1. **App-specific pause/resume (works for Amperfy)**
   ```bash
   # Pause
   mac osascript -e 'tell application "System Events" to tell process "Amperfy" to click menu item "Pause" of menu "Controls" of menu bar 1'
   # Resume
   mac osascript -e 'tell application "System Events" to tell process "Amperfy" to click menu item "Play" of menu "Controls" of menu bar 1'
   ```

2. **App-specific volume control (Apple Music/Spotify)**
   ```bash
   # Lower volume
   mac osascript -e 'tell application "Music" to set sound volume to 20'
   # Restore volume
   mac osascript -e 'tell application "Music" to set sound volume to 100'
   ```

3. **Virtual audio driver approach**
   - Use tools like BackgroundMusic for per-app volume control
   - Requires system-level audio driver installation

#### Recommended Approach:
- Detect running music apps
- Use volume control for Apple Music/Spotify
- Use pause/resume for Amperfy and unsupported apps
- Make it configurable (some users may prefer pause vs duck)

## 📝 Documentation Tasks
- [x] Create comprehensive README.md
- [x] Add usage examples
- [x] Document all CLI options
- [x] Create troubleshooting guide
- [ ] Add contribution guidelines
- [ ] Generate API documentation

## 🚀 Release Tasks
- [ ] Create GitHub Actions CI/CD
- [ ] Add release workflow
- [ ] Create homebrew formula
- [ ] Package as nix derivation
- [ ] Create demo video/gif
- [ ] Write announcement post