use std::{path, error::Error};
use configparser::ini;

use crate::init::InitError;

#[derive(Debug)]
pub struct Repository {
    pub dir: path::PathBuf,
    pub worktree: path::PathBuf,
    pub conf: ini::Ini,
}

impl Repository {
    pub fn new(path: path::PathBuf) -> Result<Self, Box<dyn Error>> {
        let path_copy = path.clone();
        let jims_dir = path_copy.join(".jims");
        if !jims_dir.exists() || !jims_dir.is_dir() {
            return Err(Box::new(InitError::from_path(jims_dir.to_str().unwrap(), "This directory is not a jims repository".to_string())))
        }
        let mut repo = Self {
            worktree: path,
            dir: path_copy.join(".jims"),
            conf: ini::Ini::new()
        };
        repo.conf.load(repo.dir.join("config"))?;
        Ok(repo)
    }
}