#!/bin/sh

# Droid Stop Hook - Voice Notifier
# This script is called when Droid finishes responding

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Load environment variables from .env file if it exists
if [ -f "$SCRIPT_DIR/.env" ]; then
    set -a
    . "$SCRIPT_DIR/.env"
    set +a
fi

# Read the JSON input from stdin
HOOK_DATA=$(cat)

# Debug logging
LOG_FILE=~/.config/voice-notifier/hook.log
mkdir -p ~/.config/voice-notifier

# Log raw input for debugging
echo "[$(date)] Raw input: $HOOK_DATA" >> "$LOG_FILE"

# Use jq for reliable JSON parsing
# Droid uses camelCase: hookEventName, transcriptPath
# Claude Code uses snake_case: hook_event_name, transcript_path
TRANSCRIPT_PATH=$(echo "$HOOK_DATA" | jq -r '.transcriptPath // .transcript_path // empty')
HOOK_EVENT_NAME=$(echo "$HOOK_DATA" | jq -r '.hookEventName // .hook_event_name // empty')
MESSAGE=$(echo "$HOOK_DATA" | jq -r '.message // empty')

echo "[$(date)] Hook triggered: $HOOK_EVENT_NAME" >> "$LOG_FILE"
echo "Script dir: $SCRIPT_DIR" >> "$LOG_FILE"
echo "Transcript path: $TRANSCRIPT_PATH" >> "$LOG_FILE"
echo "Hook event: $HOOK_EVENT_NAME" >> "$LOG_FILE"
echo "Message: $MESSAGE" >> "$LOG_FILE"
echo "ElevenLabs API key present: $([ -n "$ELEVEN_LABS_API_KEY" ] && echo "yes" || echo "no")" >> "$LOG_FILE"
echo "Anthropic API key present: $([ -n "$ANTHROPIC_API_KEY" ] && echo "yes" || echo "no")" >> "$LOG_FILE"

# Find the binary
if [ -x "$SCRIPT_DIR/target/release/voice-notifier" ]; then
    BINARY="$SCRIPT_DIR/target/release/voice-notifier"
elif [ -x "$SCRIPT_DIR/target/debug/voice-notifier" ]; then
    BINARY="$SCRIPT_DIR/target/debug/voice-notifier"
else
    echo "Error: voice-notifier binary not found!" >> "$LOG_FILE"
    echo '{"decision": "approve"}'
    exit 0
fi

# Handle different hook types
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    case "$HOOK_EVENT_NAME" in
        "Stop")
            nohup "$BINARY" --transcript "$TRANSCRIPT_PATH" --hook-event "$HOOK_EVENT_NAME" --agent-name "Droid" >> "$LOG_FILE" 2>&1 < /dev/null &
            ;;
        
        "Notification")
            if [ -z "$MESSAGE" ] || [ "$MESSAGE" = "Claude is waiting for your input" ] || [ "$MESSAGE" = "Droid is waiting for your input" ]; then
                echo "Skipping idle timeout notification (already notified by Stop hook)" >> "$LOG_FILE"
            else
                echo "Processing permission request: $MESSAGE" >> "$LOG_FILE"
                nohup "$BINARY" --transcript "$TRANSCRIPT_PATH" --hook-event "$HOOK_EVENT_NAME" --hook-message "$MESSAGE" --agent-name "Droid" >> "$LOG_FILE" 2>&1 < /dev/null &
            fi
            ;;
        
        *)
            echo "Unknown hook event: $HOOK_EVENT_NAME" >> "$LOG_FILE"
            ;;
    esac
else
    case "$HOOK_EVENT_NAME" in
        "Stop")
            nohup "$BINARY" -s "Droid has finished the task" --agent-name "Droid" >> "$LOG_FILE" 2>&1 < /dev/null &
            ;;
        
        "Notification")
            if [ -z "$MESSAGE" ] || [ "$MESSAGE" = "Claude is waiting for your input" ] || [ "$MESSAGE" = "Droid is waiting for your input" ]; then
                echo "Skipping idle timeout notification (already notified by Stop hook)" >> "$LOG_FILE"
            else
                nohup "$BINARY" -s "Droid needs your attention" --agent-name "Droid" >> "$LOG_FILE" 2>&1 < /dev/null &
            fi
            ;;
        
        *)
            echo "Unknown hook event: $HOOK_EVENT_NAME" >> "$LOG_FILE"
            ;;
    esac
fi

# Return decision to approve Droid to stop
echo '{"decision": "approve"}'
