use clap::Clap;
use indoc::printdoc;

#[derive(Clap)]
pub struct Opts {
    root: String,
}

pub fn run(opts: Opts) {
    printdoc! {
        r#"
        fs() {{
            _flow_dir=$(command flow --root "{root}" search "$@")
            _flow_ret=$?
            [ "$_flow_dir" != "$PWD" ] && cd "$_flow_dir"
            return $_flow_ret
        }}
        fsp() {{
            _flow_dir=$(command flow --root "{root}" search --project "$@")
            _flow_ret=$?
            [ "$_flow_dir" != "$PWD" ] && cd "$_flow_dir"
            return $_flow_ret
        }}
        fp() {{
            _flow_dir=$(command flow --root "{root}" clone "$@")
            _flow_ret=$?
            [ "$_flow_dir" != "$PWD" ] && cd "$_flow_dir"
            return $_flow_ret
        }}
        _flow_precmd() {{
            (command flow add "${{PWD:A}}" &)
        }}
        [[ -n "${{precmd_functions[(r)_flow_precmd]}}" ]] || {{
            precmd_functions[$(($#precmd_functions+1))]=_flow_precmd
        }}
        "#,
        root = opts.root
    }
}
