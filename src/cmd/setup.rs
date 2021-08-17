use clap::Clap;
use indoc::printdoc;

#[derive(Clap)]
pub struct Opts {
    root: String,
    #[clap(short, long)]
    path: Option<String>,
}

pub fn run(opts: Opts) {
    let path = opts.path.unwrap_or_else(|| "command flow".to_string());
    printdoc! {
        r#"
        fs() {{
            _flow_dir=$({path} --root "{root}" search "$@")
            _flow_ret=$?
            [ "$_flow_dir" != "$PWD" ] && cd "$_flow_dir"
            return $_flow_ret
        }}
        fsp() {{
            _flow_dir=$({path} --root "{root}" search --project "$@")
            _flow_ret=$?
            [ "$_flow_dir" != "$PWD" ] && cd "$_flow_dir"
            return $_flow_ret
        }}
        fp() {{
            _flow_dir=$({path} --root "{root}" clone "$@")
            _flow_ret=$?
            [ "$_flow_dir" != "$PWD" ] && cd "$_flow_dir"
            return $_flow_ret
        }}
        _flow_precmd() {{
            ({path} add "${{PWD:A}}" &)
        }}
        [[ -n "${{precmd_functions[(r)_flow_precmd]}}" ]] || {{
            precmd_functions[$(($#precmd_functions+1))]=_flow_precmd
        }}
        "#,
        root = opts.root,
        path = path
    }
}
