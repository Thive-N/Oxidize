use std::collections::HashSet;
/// deals with the .oxide file
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn check_oxide_file_exists() -> bool {
    PathBuf::from(".oxide").exists()
}

pub fn create_oxide_file(files: Vec<PathBuf>, force: bool) -> io::Result<()> {
    if check_oxide_file_exists() {
        if !force {
            println!(".oxide file already exists delete and try again");
            return Ok(());
        }
        println!("Overwriting existing .oxide file");
        fs::remove_file(".oxide")?;
    }

    let mut file = fs::File::create(".oxide")?;
    for path in files {
        writeln!(file, "{}", path.to_string_lossy())?;
    }
    Ok(())
}

pub fn read_oxide_file() -> io::Result<Vec<PathBuf>> {
    let content = fs::read_to_string(".oxide")?;
    let paths = content
        .lines()
        .map(|line| PathBuf::from(line.trim()))
        .collect();

    Ok(paths)
}

// expands all directories in the given paths and returns a flat list of file paths as a hashset
pub fn clean_paths(paths: Vec<PathBuf>) -> HashSet<PathBuf> {
    let mut set = HashSet::new();
    for path in paths {
        if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry.unwrap();
                if entry.path().is_file() {
                    set.insert(entry.path().to_path_buf());
                }
            }
        } else {
            set.insert(path);
        }
    }
    set
}
