#!/bin/sh

# Claude Stop Hook - Voice Notifier
# This script is called when Claude finishes responding

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Load environment variables from .env file if it exists
if [ -f "$SCRIPT_DIR/.env" ]; then
    # Export variables from .env file
    set -a
    . "$SCRIPT_DIR/.env"
    set +a
fi

# Read the JSON input from stdin
HOOK_DATA=$(cat)

# Extract transcript path using jq (or simple grep/sed for now)
TRANSCRIPT_PATH=$(echo "$HOOK_DATA" | grep -o '"transcript_path":"[^"]*"' | cut -d'"' -f4)

# Debug logging (actual location: ~/.config/voice-notifier/hook.log)
echo "[$(date)] Stop hook triggered" >> ~/.config/voice-notifier/hook.log
echo "Script dir: $SCRIPT_DIR" >> ~/.config/voice-notifier/hook.log
echo "Transcript path: $TRANSCRIPT_PATH" >> ~/.config/voice-notifier/hook.log
echo "ElevenLabs API key present: $([ -n "$ELEVEN_LABS_API_KEY" ] && echo "yes" || echo "no")" >> ~/.config/voice-notifier/hook.log
echo "Anthropic API key present: $([ -n "$ANTHROPIC_API_KEY" ] && echo "yes" || echo "no")" >> ~/.config/voice-notifier/hook.log

# If we have a transcript path, process it
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    # Use the compiled binary to process the transcript
    # The binary will handle parsing, summarization, and TTS
    # Try release binary first, fall back to debug
    if [ -x "$SCRIPT_DIR/target/release/voice-notifier" ]; then
        BINARY="$SCRIPT_DIR/target/release/voice-notifier"
    elif [ -x "$SCRIPT_DIR/target/debug/voice-notifier" ]; then
        BINARY="$SCRIPT_DIR/target/debug/voice-notifier"
    else
        echo "Error: voice-notifier binary not found!" >> ~/.config/voice-notifier/hook.log
        exit 1
    fi
    
    "$BINARY" --transcript "$TRANSCRIPT_PATH" 2>&1 >> ~/.config/voice-notifier/hook.log || \
    echo "Failed to run voice notifier" >> ~/.config/voice-notifier/hook.log
fi

# Return decision to approve Claude to stop
echo '{"decision": "approve"}'