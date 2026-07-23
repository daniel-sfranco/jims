use std::{path::PathBuf, error::Error};
use configparser::ini;

use crate::init::InitError;

#[derive(Debug)]
pub struct Repository {
    pub dir: PathBuf,
    pub worktree: PathBuf,
    pub conf: ini::Ini,
}

impl Repository {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
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

pub fn repo_find(path: PathBuf, required: bool) -> Result<Option<PathBuf>, String> {
    let mut path = path.clone();
    if path.is_relative() {
        path = path.canonicalize().unwrap();
    }

    let jims = path.join(".jims");
    if jims.exists() && jims.is_dir() {
        return Ok(Some(path))
    }

    let parent = path.join("..");
    if parent == path {
        if required {
            return Err("Not a git directory".to_string())
        } else {
            return Ok(None)
        }
    }

    repo_find(parent, required)
}