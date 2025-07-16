#!/bin/sh

# Claude Stop Hook - Voice Notifier
# This script is called when Claude finishes responding

# Read the JSON input from stdin
HOOK_DATA=$(cat)

# Extract transcript path using jq (or simple grep/sed for now)
TRANSCRIPT_PATH=$(echo "$HOOK_DATA" | grep -o '"transcript_path":"[^"]*"' | cut -d'"' -f4)

# Debug logging (actual location: ~/.config/voice-notifier/hook.log)
echo "[$(date)] Stop hook triggered" >> ~/.config/voice-notifier/hook.log
echo "Transcript path: $TRANSCRIPT_PATH" >> ~/.config/voice-notifier/hook.log

# If we have a transcript path, process it
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    # Use the compiled binary to process the transcript
    # The binary will handle parsing, summarization, and TTS
    /Users/tony/Dev/Personal/voice-notifier/target/debug/voice-notifier \
        --transcript "$TRANSCRIPT_PATH" 2>&1 >> ~/.config/voice-notifier/hook.log || \
    echo "Failed to run voice notifier" >> ~/.config/voice-notifier/hook.log
fi

# Return decision to approve Claude to stop
echo '{"decision": "approve"}'