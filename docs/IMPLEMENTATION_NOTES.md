# Implementation Notes

This document captures important implementation details and lessons learned during development that are crucial for future maintenance and troubleshooting.

## Critical Implementation Details

### 1. Nix-Shell Temp Directory Issue with afplay

**Problem**: When running in nix-shell, audio files created in standard temp directories may cause `afplay` to report "wht?" errors.

**Root Cause**: The nix-shell environment creates temporary directories with special permissions that can interfere with `afplay`'s ability to read files.

**Solution**: 
- Always use absolute paths when calling `afplay`
- Consider using a stable directory like `~/.config/voice-notifier/temp/` instead of system temp directories
- Ensure proper file permissions (644) on generated audio files

### 2. Claude Stop Hook JSON Format Requirements

**Critical**: The Stop hook must return a specific JSON format to work correctly.

**Correct Format**:
```json
{"decision": "approve"}
```

**Common Mistakes**:
- Using `"allow"` instead of `"approve"` - this will cause the hook to fail silently
- Missing the JSON response entirely - Claude will wait indefinitely
- Malformed JSON - Claude will ignore the hook

### 3. Shell Script Shebang Requirements

**Important**: Use `#!/bin/sh` instead of `#!/bin/bash` for the Claude hook script.

**Reason**: The Claude Code environment may not have bash available in all contexts, but `/bin/sh` is guaranteed to be present.

### 4. Transcript Path Location

**Format**: The transcript path is provided in the Stop hook JSON input:
```json
{
  "transcript_path": "/path/to/transcript.jsonl",
  "other_fields": "..."
}
```

**Notes**:
- The transcript is in JSONL format (one JSON object per line)
- Each line contains a message with `role` and `content` fields
- Look for the last line with `"role":"assistant"` for Claude's final response

### 5. Hook Log Location

**Actual Location**: `~/.config/voice-notifier/hook.log`

**Note**: This differs from the initial test location of `~/claude_hook_debug.log`. The production hook writes to the config directory for better organization.

### 6. Binary Path Considerations

**Development**: During development, the binary is at:
```
/Users/tony/Dev/Personal/voice-notifier/target/debug/voice-notifier
```

**Production**: Should be installed to a stable location like:
```
~/.local/bin/voice-notifier
```

**Important**: The hook script must use absolute paths to the binary, not relative paths or `cargo run`.

## Debugging Tips

### Hook Not Triggering
1. Check Claude Code settings for correct hook path
2. Verify hook script is executable: `chmod +x claude_stop_hook.sh`
3. Check hook.log for execution traces
4. Ensure the script returns valid JSON

### Audio Playback Issues
1. Check system volume is not muted
2. Verify `afplay` is accessible: `mac afplay --help`
3. Check audio file permissions
4. Look for "wht?" errors indicating path issues

### API Failures
1. Verify `.env` file contains valid API keys
2. Check network connectivity
3. Monitor rate limits for both ElevenLabs and Anthropic APIs
4. Test fallback chain with `--force-say` flag

## Future Improvements

### Robustness
- Add retry logic for API calls
- Implement better error recovery in the hook script
- Create a daemon mode to avoid startup overhead

### Features
- Parse and summarize transcript content
- Support multiple notification types
- Add voice profile management
- Create notification history

### Performance
- Pre-cache common messages
- Optimize API calls with batching
- Reduce binary size for faster loading