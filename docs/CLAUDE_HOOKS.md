# Claude Code Hooks Integration

## Overview
Claude Code supports hooks that execute shell commands in response to events. The hook receives context about the current session via environment variables and stdin.

## Hook Configuration
Hooks are configured in Claude Code settings under the "hooks" section.

## Available Hook Types
- `user-prompt-submit-hook`: Executes after user submits a prompt
- `assistant-response-hook`: Executes after Claude responds
- Other hooks may be available

## Hook Context
When a hook executes, it receives:
1. **Environment variables** with session context
2. **JSON data via stdin** with the current message/response

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