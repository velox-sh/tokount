# 8 lines 4 code 3 comments 1 blank
#!/usr/bin/env zsh

# Load completions
autoload -Uz compinit
compinit
setopt HIST_IGNORE_DUPS
export PATH="$HOME/bin:$PATH"
