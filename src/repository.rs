use std::path;
use configparser::ini;

#[derive(Debug)]
pub struct Repository {
    pub dir: path::PathBuf,
    pub worktree: path::PathBuf,
    pub conf: ini::Ini,
}