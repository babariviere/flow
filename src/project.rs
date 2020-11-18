use std::str::FromStr;

pub enum Project {
    Github { owner: String, repo: String },
    Git(String),
}

#[derive(Debug)]
pub enum ParseError {
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
