function fs -d "fast switch to directory"
    set -l _flow_dir ($flow_cmd --root $flow_root search $argv)
    set -l _flow_pid $last_pid
    if "$_flow_dir" != (pwd)
        cd "$_flow_dir"
    end
    return $_flow_pid
end
