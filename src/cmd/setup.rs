use clap::Clap;
use indoc::printdoc;

#[derive(Clap)]
pub struct Opts {
    root: String,
    #[clap(short, long)]
    path: Option<String>,
    #[clap(subcommand)]
    shell: Shell,
}

#[derive(Clap)]
enum Shell {
    Zsh,
    Fish,
}

pub fn run(opts: Opts) {
    let path = opts.path.unwrap_or_else(|| "command flow".to_string());
    match &opts.shell {
        Shell::Zsh => printdoc! {
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
        },
        Shell::Fish => printdoc! {
            r#"
		function fs -d "fast switch to directory"
			set -l _flow_dir ({path} --root {root} search $argv)
			set -l _flow_pid $last_pid
			if [ "$_flow_dir" != (pwd) ]
				cd "$_flow_dir"
			end
			return $_flow_pid
		end
		function fp -d "fast clone project"
			set -l _flow_dir ({path} --root {root} clone $argv)
			set -l _flow_pid $last_pid
			if [ "$_flow_dir" != (pwd) ]
				cd "$_flow_dir"
			end
			return $_flow_pid
		end
		function fsp -d "fast switch to project"
			set -l _flow_dir ({path} --root {root} search --project $argv)
			set -l _flow_pid $last_pid
			if [ "$_flow_dir" != (pwd) ]
				cd "$_flow_dir"
			end
			return $_flow_pid
		end
		function _flow_add_directory --on-event fish_prompt
			{path} add "$PWD" &
		end
		"#,
            root = opts.root,
            path = path
        },
    }
}
