use std::path::{Path, PathBuf};

use plc::Target;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    path: PathBuf,
    target: Target,
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl From<PathBuf> for Object {
    fn from(path: PathBuf) -> Self {
        Object { path, target: Target::System }
    }
}

impl Object {
    pub fn with_target(mut self, target: &Target) -> Self {
        self.target = target.clone();
        self
    }
    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_target(&self) -> &Target {
        &self.target
    }
}
