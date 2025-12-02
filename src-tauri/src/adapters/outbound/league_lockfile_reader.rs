use crate::domain::ports::{LockfileData, LockfilePort};
use anyhow::Result;
use std::{fs, path::PathBuf};

pub struct LeagueLockfileReader {
    path: PathBuf,
}

impl LeagueLockfileReader {
    pub fn new(path: Option<PathBuf>) -> Self {
        let default = PathBuf::from("C:/Riot Games/League of Legends/lockfile");
        Self {
            path: path.unwrap_or(default),
        }
    }
}

impl LockfilePort for LeagueLockfileReader {
    fn read_lockfile(&self) -> Result<LockfileData> {
        let content = fs::read_to_string(&self.path)?;
        let parts: Vec<&str> = content.trim().split(':').collect();

        Ok(LockfileData {
            port: parts[2].parse()?,
            password: parts[3].to_string(),
        })
    }
}
