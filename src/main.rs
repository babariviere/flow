mod cache;
mod cmd;
mod util;

use clap::{AppSettings, Clap};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command as PCommand;
use std::str::FromStr;

use crate::cmd::*;
use crate::util::read_cache;

#[derive(Clap)]
#[clap(author, about, version)]
struct Opts {
    /// Root directory that contains all projects.
    /// Defaults to $HOME/src.
    #[clap(short, long)]
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
    Search {
        #[clap(short, long)]
        project: bool,
        path: Vec<String>,
    },
    /// Clone a project
    Clone { project: String },
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
        Command::Search { project, path } => search(root, project, path.join(" ")),
        Command::Clone { project } => clone(root, project),
        Command::Add(opts) => add::run(opts),
    }
}

fn search(root: String, project: bool, query: String) {
    // TODO: handle error
    let cache = read_cache().unwrap();

    let mut dirs: cache::Cache = list_files(&root, 2)
        .into_iter()
        .map(|path| format!("{}/{}", root, path))
        .map(|path| {
            let score = if project {
                999.
            } else {
                cache.get(&path).map(|val| val.score()).unwrap_or(0.)
            };
            (path, cache::CacheEntry::new(score))
        })
        .collect();

    if !project {
        dirs.extend(cache.into_iter());
    }

    let mut result = dirs
        .iter()
        .map(|(path, entry)| {
            let score = score_query(&query, &path);
            (score, path, entry.score())
        })
        .collect::<Vec<(i32, &String, f32)>>();

    result.sort_by(|(isa, _, sa), (isb, _, sb)| isa.cmp(isb).then(sa.partial_cmp(sb).unwrap()));

    // NOTE: for debug purpose only
    // for (score, path) in &result {
    //     println!("[{}] {}", score, path);
    // }
    // TODO: do not use unwrap
    let (_, path, _) = result.pop().unwrap();
    println!("{}", path);
}

fn list_files<P: AsRef<Path>>(path: P, depth: u32) -> Vec<String> {
    if depth == 0 {
        return fs::read_dir(path)
            .unwrap()
            .filter_map(|dir| {
                let dir = dir.ok()?;
                if dir.file_type().ok()?.is_dir() {
                    let file_name = dir.file_name().into_string().ok()?;
                    if file_name.starts_with('.') {
                        return None;
                    }
                    Some(file_name)
                } else {
                    None
                }
            })
            .collect();
    }
    fs::read_dir(path)
        .unwrap()
        .filter_map(|dir| {
            let dir = dir.ok()?;
            if dir.file_type().ok()?.is_dir() {
                let file_name = dir.file_name().into_string().ok()?;
                if file_name.starts_with('.') {
                    return None;
                }
                Some(
                    list_files(dir.path(), depth - 1)
                        .into_iter()
                        .map(|child| format!("{}/{}", file_name, child))
                        .collect::<Vec<String>>(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn score_query(query: &str, path: &str) -> i32 {
    let mut query = query.split(' ').collect::<Vec<&str>>();
    let mut items = path.split('/').collect::<Vec<&str>>();

    let query_len = query.len();
    let items_len = items.len();
    let first_query = query.drain(..1).collect::<Vec<&str>>().pop().unwrap();
    if items_len < query_len {
        return 0;
    }
    let (first_item_pos, first_score) = items
        .iter()
        .take(items_len - query_len + 1)
        .map(|item| score_part(first_query, item))
        .enumerate()
        .max_by_key(|(_, score)| *score)
        .unwrap();

    // Remove unmatchable entries
    if first_score <= 0 {
        return 0;
    }

    let mut global_score = first_score;
    let mut items = items.drain(first_item_pos + 1..);

    for query in query {
        loop {
            let item = match items.next() {
                Some(i) => i,
                None => return 0,
            };
            let score = score_part(query, item);
            // TODO: which score threshold to use?
            if score < 5 {
                continue;
            }
            global_score += score;
            break;
        }
    }

    if items.next().is_none() {
        global_score += 10;
    }

    global_score
}

fn score_part(query: &str, item: &str) -> i32 {
    let mut query_chars = query.chars();
    let mut item_chars = item.chars();

    let mut successive = 0;
    let mut score = 0;
    let mut is_first = true;

    'query: while let Some(q) = query_chars.next() {
        let c = match item_chars.next() {
            Some(c) => c,
            None => {
                score = 0;
                break;
            }
        };

        if q == c {
            score += 1 + successive * 2;
            if is_first {
                score += 4;
            }
            successive += 1;
            is_first = false;
            continue;
        }
        is_first = false;

        loop {
            let c = match item_chars.next() {
                Some(c) => c,
                None => {
                    score = 0;
                    break 'query;
                }
            };

            if q == c {
                score += 1;
                successive = 1;
                break;
            }
        }
    }

    // TODO: which score to use for best match?
    if item_chars.next().is_none() && query_chars.next().is_none() && successive > 0 {
        score += 4;
    }

    score
}

enum Project {
    Github { owner: String, repo: String },
    Git(String),
}

#[derive(Debug)]
enum ParseError {
    UnknownPrefix(String),
    InvalidGithubProject(String),
}

impl FromStr for Project {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.splitn(2, ':').collect::<Vec<&str>>();

        let (ty, project) = if parts.len() < 2 {
            ("gh", parts[0])
        } else {
            (parts[0], parts[1])
        };

        match ty {
            "gh" | "github" => {
                let parts = project.split('/').collect::<Vec<&str>>();
                if parts.len() != 2 {
                    return Err(ParseError::InvalidGithubProject(project.to_owned()));
                }
                let owner = parts[0].to_owned();
                let repo = parts[1].trim_end_matches(".git").to_owned();

                Ok(Self::Github { owner, repo })
            }
            // TODO: parse git url into:
            // - protocol (ssh?) or maybe full url?
            // - host
            // - path
            "git" => Ok(Self::Git(project.to_string())),
            ty => Err(ParseError::UnknownPrefix(ty.to_owned())),
        }
    }
}

fn clone(root: String, project: String) {
    let project = match Project::from_str(&project) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    match project {
        Project::Github { owner, repo } => {
            let repo_url = format!("git@github.com:{}/{}.git", owner, repo);
            let out = format!("{}/github.com/{}/{}", root, owner, repo);

            if Path::new(&out).exists() {
                println!("{}", out);
                return;
            }
            use std::os::unix::io::{AsRawFd, FromRawFd};
            let stderr_fd =
                unsafe { std::process::Stdio::from_raw_fd(std::io::stderr().as_raw_fd()) };

            PCommand::new("git")
                .args(&[
                    "clone",
                    "--recursive",
                    "--",
                    repo_url.as_str(),
                    out.as_str(),
                ])
                .stdout(stderr_fd)
                .status()
                .expect("should be ok");
            println!("{}", out);
        }
        Project::Git(_url) => {}
    }
}
