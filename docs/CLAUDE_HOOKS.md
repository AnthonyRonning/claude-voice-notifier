# Claude Code Hooks Integration

## Overview
Claude Code supports hooks that execute shell commands in response to events. The hook receives context about the current session via environment variables and stdin.

## Hook Configuration
Hooks are configured in Claude Code settings under the "hooks" section.

## Available Hook Types
- `Stop`: Executes when Claude finishes responding
- `Notification`: Executes when Claude needs permission or after 60 seconds of idle time
- `user-prompt-submit-hook`: Executes after user submits a prompt  
- `assistant-response-hook`: Executes after Claude responds
- Other hooks may be available

## Hook Requirements
**CRITICAL**: Both Stop and Notification hooks must return a JSON decision:
```json
{"decision": "approve"}
```
- Must use `"approve"` not `"allow"`
- Must be valid JSON
- Must use `#!/bin/sh` shebang (not `#!/bin/bash`)

## Hook Context
When a hook executes, it receives:
1. **Environment variables** with session context
2. **JSON data via stdin** with the current message/response

### Hook JSON Input
Both hooks receive JSON with:
```json
{
  "transcript_path": "/path/to/conversation.jsonl",
  "hook_event_name": "Stop" | "Notification",
  "message": "Optional message (for Notification hook)",
  "session_id": "unique-session-identifier"
}
```

### Notification Hook Triggers
The Notification hook triggers in two scenarios:
1. **Permission Required**: When Claude needs permission to use a tool (message: "Claude needs your permission to use [tool]")
2. **Idle Timeout**: When the prompt input has been idle for 60+ seconds (message: "Claude is waiting for your input")

The transcript file contains JSONL (one JSON per line) with messages:
```json
{"role": "user", "content": "User's message"}
{"role": "assistant", "content": "Claude's response"}
```

## Implementation
The voice notifier now supports both hooks:

### Stop Hook Flow
1. Parse the JSON input from stdin
2. Extract Claude's last response from transcript
3. Use Claude 4 Sonnet to summarize what was completed
4. Generate speech with ElevenLabs
5. Play the summary audio

### Notification Hook Flow
1. Parse the JSON input from stdin
2. Extract Claude's current context from transcript
3. Use Claude 4 Sonnet to summarize what Claude needs/is waiting for
4. Generate speech with ElevenLabs
5. Play the context-aware notification

### Context-Aware Summaries
The summarizer uses different prompts based on the event type:
- **Stop events**: Focus on what Claude completed
- **Notification events**: Focus on what Claude needs or is waiting for

## Implemented Features
- ✅ JSON parsing for hook input
- ✅ Anthropic API client for summarization (Claude 4 Sonnet)
- ✅ Integration with existing TTS system
- ✅ Support for both Stop and Notification hooks
- ✅ Context-aware summaries based on event type
- ✅ Fallback to simple messages when transcript unavailable
- ✅ Automatic loading of environment variables from .env