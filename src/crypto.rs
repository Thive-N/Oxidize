use crate::PathBuf;
use crate::oxide::{clean_paths, read_oxide_file};

pub fn decrypt(files: Vec<PathBuf>, password: Option<String>, keep: bool) {
    let raw_files: Vec<PathBuf> = if files.is_empty() {
        read_oxide_file().expect("Failed to read .oxide file")
    } else {
        files
    };
    let target_files = clean_paths(raw_files);
    let pwd = match password {
        Some(p) => p,
        _ => rpassword::prompt_password("enter password> ").unwrap(),
    };

    println!(
        "files: {:?}, password: {:?}, keep: {}",
        target_files, pwd, keep
    );
}
