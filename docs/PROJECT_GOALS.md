# Voice Notifier Project Goals

## Overview
A Rust-based voice notification system for Claude Code completions, using ElevenLabs TTS with macOS `say` fallback.

## Core Features

### Phase 1: MVP
- Simple CLI tool that plays voice notifications
- ElevenLabs TTS integration for high-quality voices
- Fallback to macOS `say` command if ElevenLabs fails
- Basic "Task complete" notification
- Configuration via `.env` file for API keys

### Phase 2: Enhanced Notifications
- Detailed task completion messages
- Parse Claude Code output to extract task information
- Different notification types (success, error, warning)
- Customizable voice selection

### Phase 3: Advanced Features
- Claude Code hook integration
- Custom message templates
- Voice profile management
- Notification history/logging

## Technical Requirements

### Environment
- Runs in OrbStack VM on macOS
- Uses `mac afplay` for audio playback
- Built with Rust for performance and reliability
- Nix flake for reproducible development environment

### Configuration
- `.env` file for sensitive data (API keys)
- Command-line arguments for runtime options
- JSON/TOML config for voice preferences and templates

### Error Handling
- Graceful fallback to `say` command
- Clear error messages for debugging
- Non-blocking execution (Claude Code shouldn't wait)

## Integration Points

### Claude Code Hooks
- Execute on task completion
- Pass task context to notifier
- Handle various completion states

### Audio Pipeline
1. Receive notification request
2. Generate audio via ElevenLabs API
3. Save temporary audio file
4. Play using `mac afplay`
5. Clean up temporary files

## Success Criteria
- Zero-configuration setup after initial `.env` creation
- Sub-second notification delivery
- 100% reliability with fallback mechanism
- Minimal resource usage