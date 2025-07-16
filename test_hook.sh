#!/bin/bash

# Test script to capture Claude hook data
echo "=== HOOK TRIGGERED ===" >> ~/claude_hook_debug.log
echo "Date: $(date)" >> ~/claude_hook_debug.log
echo "Environment Variables:" >> ~/claude_hook_debug.log
env | grep -E "CLAUDE|HOOK" >> ~/claude_hook_debug.log
echo "Stdin Data:" >> ~/claude_hook_debug.log
cat >> ~/claude_hook_debug.log
echo -e "\n=== END HOOK ===\n" >> ~/claude_hook_debug.log

# Also save to a separate file for easier parsing
cat > ~/last_claude_hook.json