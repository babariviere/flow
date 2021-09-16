set -q flow_cmd || set flow_cmd flow
set -q flow_root || set flow_root "$HOME/src"

function _flow_add_directory --on-event fish_prompt
    $flow_cmd add "$PWD" &
end
