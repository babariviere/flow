mod cache;
mod cmd;
mod project;
mod util;

use clap::{AppSettings, Clap};
use std::env;

use crate::cmd::*;

#[derive(Clap)]
#[clap(author, about, version)]
struct Opts {
    /// Root directory that contains all projects.
    /// Defaults to $HOME/src.
    #[clap(short, long, env = "FLOW_ROOT")]
    root: Option<String>,
    #[clap(subcommand)]
    cmd: Command,
}

// TODO: add search in project (search for git root and search inside)
// TODO: cleanup code (split by subcommands + add a module for cache and another one for scoring)

#[derive(Clap)]
pub enum Command {
    /// Creates a shell script to use in your project.
    /// Usage: eval "$(flow setup [root])"
    Setup(setup::Opts),
    /// Search for a project in root directory.
    Search(search::Opts),
    /// Clone a project
    Clone(clone::Opts),
    /// Adds a path to the cache
    #[clap(setting = AppSettings::Hidden)]
    Add(add::Opts),
}

fn main() {
    let opts = Opts::parse();

    let root = opts.root.unwrap_or_else(|| {
        let mut home = env::var("HOME").expect("HOME is not defined.");
        home.push_str("/src");
        home
    });

    match opts.cmd {
        Command::Setup(opts) => setup::run(opts),
        Command::Search(opts) => search::run(root, opts),
        Command::Clone(opts) => clone::run(root, opts),
        Command::Add(opts) => add::run(opts),
    }
}
