# Voice Notifier for Claude Code

A Rust-based voice notification system that announces when Claude Code completes tasks using ElevenLabs TTS.

## Features

- 🔊 High-quality voice notifications using ElevenLabs API
- 🔄 Automatic fallback to macOS `say` command
- 🪝 Claude Code Stop hook integration
- ⚡ Fast and reliable Rust implementation
- 🏗️ Nix flake for reproducible builds

## Current Status

✅ **Phase 1 Complete**: Basic voice notifications are working!
- ElevenLabs TTS integration
- Claude Code hook triggers successfully
- Fallback chain implemented

🚧 **Phase 2 In Progress**: Intelligent summaries
- Parse Claude's responses from transcript
- Summarize with Anthropic API
- Context-aware notifications

## Installation

1. Clone the repository:
```bash
git clone <repo-url>
cd voice-notifier
```

2. Enter the Nix development shell:
```bash
nix develop
```

3. Build the project:
```bash
cargo build --release
```

4. Create a `.env` file:
```bash
cp .env.example .env
# Edit .env with your API keys
```

## Configuration

### Environment Variables

Create a `.env` file with:
```
ELEVEN_LABS_API_KEY=your_key_here
ELEVEN_LABS_VOICE_ID=voice_id_here  # Optional
ANTHROPIC_API_KEY=your_anthropic_key  # For Phase 2
```

### Claude Code Hook Setup

1. Add to your Claude Code settings:
```json
{
  "hooks": {
    "stop": "/path/to/voice-notifier/claude_stop_hook.sh"
  }
}
```

2. The hook will trigger voice notifications when Claude finishes responding.

## Usage

### Manual Testing
```bash
# Test with default message
cargo run -- --test

# Custom message
cargo run -- -s "Build completed successfully"

# Play specific audio file
cargo run -- -f audio.mp3

# Force fallback to say command
cargo run -- --force-say -s "Testing fallback"
```

### With Claude Code
Once configured, the voice notifier automatically announces when Claude completes tasks.

## Architecture

```
Claude Code → Stop Hook → voice-notifier
                              ↓
                        ElevenLabs API
                              ↓
                     [Audio File] → afplay
                              ↓
                        (Fallback: mac say)
```

## Troubleshooting

- **"wht?" error**: Usually a path issue with nix-shell temp directories
- **No sound**: Check your system volume and `.env` configuration
- **Hook not triggering**: Verify the path in Claude Code settings

## Development

```bash
# Run with debug logging
RUST_LOG=voice_notifier=debug cargo run

# Run tests
cargo test

# Check code
cargo clippy
cargo fmt
```

## License

MIT