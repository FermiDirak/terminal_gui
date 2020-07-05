use std::fs;
use std::path::{Path, PathBuf};

pub const APP_DATA_DIRECTORY_ROOT: &str = "~/.vivian";

pub fn get_or_build_file(path: &Path) -> fs::File {
    let parent = path.parent().unwrap();
    fs::create_dir_all(&parent).unwrap();
    if !path.exists() {
        return fs::File::create(&path).unwrap();
    }
    fs::File::open(&path).unwrap()
}
