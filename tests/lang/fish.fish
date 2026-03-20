# 9 lines 5 code 3 comments 1 blank
#!/usr/bin/env fish

# Set environment
set -x EDITOR nvim
set -x PATH $HOME/bin $PATH
function greet
    echo "Hello, $argv!"
end
