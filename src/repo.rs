use std::{fs::DirEntry, path::PathBuf};

pub struct Repo {
    name: String,
    path: PathBuf,
}

type Error = std::io::Error;

impl TryFrom<DirEntry> for Repo {
    type Error = Error;

    fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
        Ok(Repo {
            name: value.file_name().into_string().unwrap(),
            path: value.path(),
        })
    }
}

impl Repo {
    pub fn status(&self) -> Result<String, Error> {
        Ok(String::from("Clean"))
    }
}
