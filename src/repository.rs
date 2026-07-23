use std::{path, error::Error};
use configparser::ini;

#[derive(Debug)]
pub struct Repository {
    pub dir: path::PathBuf,
    pub worktree: path::PathBuf,
    pub conf: ini::Ini,
}

impl Repository {
    pub fn new(path: path::PathBuf) -> Result<Self, Box<dyn Error>> {
        let path_copy = path.clone();
        let mut repo = Self {
            worktree: path,
            dir: path_copy.join(".jims"),
            conf: ini::Ini::new()
        };
        repo.conf.load(repo.dir.join("config"))?;
        Ok(repo)
    }
}