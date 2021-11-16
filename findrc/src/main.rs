use std::path::{Path, PathBuf};
use std::{env, str};

const PATHRC_FILENAME: &str = ".path-rc";

fn main() {
    let mut found_files: Vec<String> = Vec::new();
    let mut dir = get_path().unwrap();

    while let Some(path_buf) = find_pathrc(&dir) {
        let p_dir = next_dir(path_buf.as_path());
        if p_dir == None {
            break;
        } else {
            dir = p_dir.unwrap();
        }
        let p_file = path_buf.to_string_lossy() + "/.path-rc";
        found_files.push(p_file.to_string());
    }

    while let Some(top) = found_files.pop() {
        println!("{}", top);
    }
}

fn next_dir(current_dir: &Path) -> Option<PathBuf> {
    if current_dir == PathBuf::from("/").as_path() {
        None
    } else {
        Some(PathBuf::from(current_dir.parent().unwrap()))
    }
}

fn get_path() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

fn find_pathrc(starting_directory: &Path) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(PATHRC_FILENAME);

    loop {
        path.push(file);
        if path.is_file() {
            let path_dir = path.parent();
            let buf = PathBuf::from(path_dir.unwrap());
            break Some(buf);
        }

        if !(path.pop() && path.pop()) {
            // remove file && remove parent
            break None;
        }
    }
}
