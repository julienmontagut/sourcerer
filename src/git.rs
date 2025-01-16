use std::fmt;
use std::fs::DirEntry;
use std::path::PathBuf;

use serde::de::value;

type Error = std::io::Error;

pub struct Repository {
    name: String,
    url: String,
    path: PathBuf,
}

pub enum Status {
    Clean,
    Dirty,
    Ahead,
    Behind,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Clean => write!(f, "Clean"),
            Status::Dirty => write!(f, "Dirty"),
            Status::Ahead => write!(f, "Ahead"),
            Status::Behind => write!(f, "Behind"),
        }
    }
}

impl Repository {
    pub fn status(&self) -> Result<Status, Box<dyn std::error::Error>> {
        let repo = git2::Repository::open(&self.path)?;
        let status = repo.statuses(None)?;
        if status.is_empty() {
            Ok(Status::Clean)
        } else {
            Ok(Status::Dirty)
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl TryFrom<PathBuf> for Repository {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if path.is_dir() && path.join(".git").is_dir() {
            let repository = Repository {
                name: "name".to_string(),
                url: "url".to_string(),
                path: path,
            };
            Ok(repository)
        } else {
            Err(Error::new(
                std::io::ErrorKind::NotFound,
                "Git repository not found",
            ))
        }
    }
}
