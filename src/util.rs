use std::path::Path;
use std::fs::metadata;

/// Does the directory or file exist?
pub fn exists(path: &Path) -> bool {
    metadata(path).is_err()
}

pub fn mkpath(path: &str) -> &Path {
    Path::new(path)
}
