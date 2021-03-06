use clap::Clap;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use crate::project::Project;

#[derive(Clap)]
pub struct Opts {
    project: String,
}

pub fn run(root: String, opts: Opts) {
    let project = match Project::from_str(&opts.project) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    match project {
        Project::Github { owner, repo } => {
            let repo_url = format!("git@github.com:{}/{}.git", owner, repo);
            clone_git_project(&root, &repo_url, "github.com", &owner, &repo);
        }
        Project::Git {
            url,
            domain,
            owner,
            repo,
        } => clone_git_project(&root, &url, &domain, &owner, &repo),
    }
}

fn clone_git_project(root: &str, url: &str, domain: &str, owner: &str, repo: &str) {
    let out = format!("{}/{}/{}/{}", root, domain, owner, repo);

    if Path::new(&out).exists() {
        println!("{}", out);
        return;
    }
    use std::os::unix::io::{AsRawFd, FromRawFd};
    let stderr_fd = unsafe { std::process::Stdio::from_raw_fd(std::io::stderr().as_raw_fd()) };

    Command::new("git")
        .args(&["clone", "--recursive", "--", url, out.as_str()])
        .stdout(stderr_fd)
        .status()
        .expect("should be ok");
    println!("{}", out);
}
