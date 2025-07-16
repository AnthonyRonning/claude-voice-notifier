# Claude Code Hooks Integration

## Overview
Claude Code supports hooks that execute shell commands in response to events. The hook receives context about the current session via environment variables and stdin.

## Hook Configuration
Hooks are configured in Claude Code settings under the "hooks" section.

## Available Hook Types
- `stop`: Executes when Claude finishes responding (this is what we use)
- `user-prompt-submit-hook`: Executes after user submits a prompt  
- `assistant-response-hook`: Executes after Claude responds
- Other hooks may be available

## Stop Hook Requirements
**CRITICAL**: The Stop hook must return a JSON decision:
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

### Stop Hook JSON Input
The Stop hook receives JSON with:
```json
{
  "transcript_path": "/path/to/conversation.jsonl",
  "other_fields": "..."
}
```

The transcript file contains JSONL (one JSON per line) with messages:
```json
{"role": "user", "content": "User's message"}
{"role": "assistant", "content": "Claude's response"}
```

## Implementation Plan
1. Parse the JSON input from stdin
2. Extract Claude's response text
3. Use Anthropic API to summarize the response
4. Generate speech with ElevenLabs
5. Play the summary audio

## Required Features
- JSON parsing for hook input
- Anthropic API client for summarization
- Integration with existing TTS system