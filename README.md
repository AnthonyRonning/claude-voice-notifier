# Claude Voice Notifier

A Rust-based voice notification system for Claude Code that provides intelligent audio summaries when Claude completes tasks.

## Features

- 🎯 **Intelligent Summarization**: Uses Claude 4 Sonnet to create concise summaries of completed work
- 🔊 **High-Quality Voice**: Integrates with ElevenLabs for natural text-to-speech
- 🔄 **Robust Fallbacks**: Falls back to macOS `say` command if external services fail
- 🪝 **Claude Code Integration**: Works with both Stop and Notification hooks
- 📝 **Transcript Parsing**: Automatically extracts and processes Claude's responses
- 🔔 **Smart Notifications**: Different messages for task completion vs permission requests

## How It Works

1. When Claude Code finishes a task, the stop hook triggers
2. The voice notifier reads Claude's transcript (JSONL format)
3. Extracts the last assistant message
4. Sends it to Claude 4 Sonnet for intelligent summarization
5. Converts the summary to speech using ElevenLabs
6. Plays the audio notification

## Prerequisites

- Rust (latest stable)
- macOS (for `afplay` audio playback)
- Claude Code with hooks enabled
- API Keys:
  - ElevenLabs API key (required for TTS)
  - Anthropic API key (optional, for intelligent summaries)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/AnthonyRonning/claude-voice-notifier.git
cd claude-voice-notifier
```

2. Create a `.env` file with your API keys:
```bash
cp .env.example .env
# Edit .env and add your API keys
```

3. Build the project:
```bash
cargo build --release
```

4. Set up the Claude Code hooks by adding to your Claude settings:
```json
{
  "hooks": {
    "Stop": [
      {
        "command": "/path/to/claude-voice-notifier/claude_stop_hook.sh"
      }
    ],
    "Notification": [
      {
        "command": "/path/to/claude-voice-notifier/claude_stop_hook.sh"
      }
    ]
  }
}
```

## Usage

### Manual Testing
```bash
# Test with default message
cargo run -- --test

# Custom message
cargo run -- -s "Build completed successfully"

# Process a Claude transcript
cargo run -- --transcript /path/to/transcript.jsonl
```

### CLI Options
- `-s, --text <TEXT>`: Text to speak
- `-f, --file <FILE>`: Audio file to play
- `--test`: Test mode with default notification
- `--transcript <PATH>`: Process a Claude transcript file
- `--force-say`: Force use of macOS say command
- `--keep-temp`: Keep temporary files for debugging

## Configuration

Environment variables (via `.env`):
- `ELEVEN_LABS_API_KEY`: Your ElevenLabs API key (required)
- `ELEVEN_LABS_VOICE_ID`: Voice ID (defaults to "Rachel")
- `ELEVEN_LABS_MODEL_ID`: Model ID (defaults to "eleven_multilingual_v2")
- `ANTHROPIC_API_KEY`: Your Anthropic API key (optional, enables intelligent summaries)

## Architecture

```
Claude Code → Stop Hook → Voice Notifier
                              ↓
                    Extract Last Message
                              ↓
                    Anthropic Summarization
                              ↓
                    ElevenLabs TTS
                              ↓
                    Audio Playback
```

## Development

```bash
# Run with debug logging
RUST_LOG=voice_notifier=debug cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Troubleshooting

1. **No audio playing**: Ensure you're on macOS with `afplay` available
2. **API errors**: Check your API keys in `.env`
3. **Hook not triggering**: Verify hook permissions and path in Claude settings
4. **Check logs**: Look at `~/.config/voice-notifier/hook.log`

## License

MIT

## Acknowledgments

Built for use with [Claude Code](https://claude.ai/code) by Anthropic.