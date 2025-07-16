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

# If we have a transcript path, get the last assistant message
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    # Get the last assistant message from the JSONL file
    # Each line is a JSON object, we want the last one with role="assistant"
    LAST_ASSISTANT_MSG=$(grep '"role":"assistant"' "$TRANSCRIPT_PATH" | tail -1)
    
    if [ -n "$LAST_ASSISTANT_MSG" ]; then
        # For now, just play a simple notification
        # TODO: Extract text and summarize
        # Use the compiled binary instead of cargo run for reliability
        /Users/tony/Dev/Personal/voice-notifier/target/debug/voice-notifier \
            -s "Claude has finished the task" 2>&1 >> ~/.config/voice-notifier/hook.log || \
        echo "Failed to run voice notifier" >> ~/.config/voice-notifier/hook.log
    fi
fi

# Return decision to approve Claude to stop
echo '{"decision": "approve"}'
