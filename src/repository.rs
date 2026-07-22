use std::path;

#[derive(Debug)]
pub struct Repository {
    pub dir: path::PathBuf,
    pub worktree: path::PathBuf,
    pub conf: String,
}