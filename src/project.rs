use std::str::FromStr;
use url::Url;

pub enum Project {
    Github {
        owner: String,
        repo: String,
    },
    Git {
        url: String,
        domain: String,
        owner: String,
        repo: String,
    },
}

#[derive(Debug)]
pub enum ParseError {
    UnknownPrefix(String),
    InvalidGithubProject(String),
    Url(url::ParseError),
    MissingDomain,
    MissingOwner,
    MissingRepo,
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
            "http" | "https" | "ssh" | "git+ssh" => parse_url(s),
            prefix if prefix.contains("@") => {
                parse_url(format!("ssh://{}/{}", parts[0], parts[1]).as_str())
            }
            ty => Err(ParseError::UnknownPrefix(ty.to_owned())),
        }
    }
}

fn parse_url(url: &str) -> Result<Project, ParseError> {
    let u = match Url::parse(url) {
        Ok(u) => u,
        Err(e) => return Err(ParseError::Url(e)),
    };

    let mut path = u.path_segments().ok_or(ParseError::MissingOwner)?;
    let owner = path.next().ok_or(ParseError::MissingOwner)?.to_owned();
    let repo = path.next().ok_or(ParseError::MissingRepo)?.to_owned();

    Ok(Project::Git {
        url: url.to_owned(),
        domain: u.host_str().ok_or(ParseError::MissingDomain)?.to_owned(),
        owner,
        repo,
    })
}
