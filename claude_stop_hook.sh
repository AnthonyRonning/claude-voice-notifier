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

# Extract data from the hook JSON
TRANSCRIPT_PATH=$(echo "$HOOK_DATA" | grep -o '"transcript_path":"[^"]*"' | cut -d'"' -f4)
HOOK_EVENT_NAME=$(echo "$HOOK_DATA" | grep -o '"hook_event_name":"[^"]*"' | cut -d'"' -f4)
MESSAGE=$(echo "$HOOK_DATA" | grep -o '"message":"[^"]*"' | cut -d'"' -f4)

# Debug logging (actual location: ~/.config/voice-notifier/hook.log)
echo "[$(date)] Hook triggered: $HOOK_EVENT_NAME" >> ~/.config/voice-notifier/hook.log
echo "Script dir: $SCRIPT_DIR" >> ~/.config/voice-notifier/hook.log
echo "Transcript path: $TRANSCRIPT_PATH" >> ~/.config/voice-notifier/hook.log
echo "Hook event: $HOOK_EVENT_NAME" >> ~/.config/voice-notifier/hook.log
echo "Message: $MESSAGE" >> ~/.config/voice-notifier/hook.log
echo "ElevenLabs API key present: $([ -n "$ELEVEN_LABS_API_KEY" ] && echo "yes" || echo "no")" >> ~/.config/voice-notifier/hook.log
echo "Anthropic API key present: $([ -n "$ANTHROPIC_API_KEY" ] && echo "yes" || echo "no")" >> ~/.config/voice-notifier/hook.log

# Find the binary
if [ -x "$SCRIPT_DIR/target/release/voice-notifier" ]; then
    BINARY="$SCRIPT_DIR/target/release/voice-notifier"
elif [ -x "$SCRIPT_DIR/target/debug/voice-notifier" ]; then
    BINARY="$SCRIPT_DIR/target/debug/voice-notifier"
else
    echo "Error: voice-notifier binary not found!" >> ~/.config/voice-notifier/hook.log
    exit 1
fi

# Handle different hook types
if [ -n "$TRANSCRIPT_PATH" ] && [ -f "$TRANSCRIPT_PATH" ]; then
    # Always process the transcript when available
    case "$HOOK_EVENT_NAME" in
        "Stop")
            # For Stop hook, process normally (run in background)
            nohup "$BINARY" --transcript "$TRANSCRIPT_PATH" --hook-event "$HOOK_EVENT_NAME" --agent-name "Claude Code" >> ~/.config/voice-notifier/hook.log 2>&1 < /dev/null &
            ;;
        
        "Notification")
            # Skip idle timeout notifications (already handled by Stop hook)
            # Only process actual permission requests
            if [ -z "$MESSAGE" ] || [ "$MESSAGE" = "Claude is waiting for your input" ]; then
                echo "Skipping idle timeout notification (already notified by Stop hook)" >> ~/.config/voice-notifier/hook.log
            else
                # For Notification hook with actual permission request, process transcript with event context
                echo "Processing permission request: $MESSAGE" >> ~/.config/voice-notifier/hook.log
                nohup "$BINARY" --transcript "$TRANSCRIPT_PATH" --hook-event "$HOOK_EVENT_NAME" --hook-message "$MESSAGE" --agent-name "Claude Code" >> ~/.config/voice-notifier/hook.log 2>&1 < /dev/null &
            fi
            ;;
        
        *)
            echo "Unknown hook event: $HOOK_EVENT_NAME" >> ~/.config/voice-notifier/hook.log
            ;;
    esac
else
    # Fallback if no transcript is available
    case "$HOOK_EVENT_NAME" in
        "Stop")
            nohup "$BINARY" -s "Claude Code has finished the task" --agent-name "Claude Code" >> ~/.config/voice-notifier/hook.log 2>&1 < /dev/null &
            ;;
        
        "Notification")
            # Skip idle timeout notifications (already handled by Stop hook)
            if [ -z "$MESSAGE" ] || [ "$MESSAGE" = "Claude is waiting for your input" ]; then
                echo "Skipping idle timeout notification (already notified by Stop hook)" >> ~/.config/voice-notifier/hook.log
            else
                nohup "$BINARY" -s "Claude Code needs your attention" --agent-name "Claude Code" >> ~/.config/voice-notifier/hook.log 2>&1 < /dev/null &
            fi
            ;;
        
        *)
            echo "Unknown hook event: $HOOK_EVENT_NAME" >> ~/.config/voice-notifier/hook.log
            ;;
    esac
fi

# Return decision to approve Claude to stop
echo '{"decision": "approve"}'