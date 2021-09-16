function fp -d "fast clone project"
    set -l _flow_dir ($flow_cmd --root $flow_root clone $argv)
    set -l _flow_pid $last_pid
    if $_flow_dir != (pwd)
        cd "$_flow_dir"
    end
    return $_flow_pid
end
