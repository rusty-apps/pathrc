use std::path::{Path, PathBuf};
use std::{env, str};

const PATHRC_FILENAME: &str = ".path-rc";

pub fn get_path() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

pub struct FindRC {
    directory: PathBuf,
    found_files: Vec<String>,
}

impl FindRC {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            found_files: Vec::new(),
        }
    }

    pub fn found_files(&self) -> &Vec<String> {
        &self.found_files
    }

    pub fn search_files(&mut self) {
        while let Some(path_buf) = Self::find_pathrc(&self) {
            let p_dir = Self::next_dir(path_buf.as_path());
            if p_dir == None {
                break;
            } else {
                self.directory = p_dir.unwrap();
            }
            let p_file = path_buf.to_string_lossy() + "/.path-rc";
            self.found_files.push(p_file.to_string());
        }
    }

    fn next_dir(current_dir: &Path) -> Option<PathBuf> {
        if current_dir == PathBuf::from("/").as_path() {
            None
        } else {
            Some(PathBuf::from(current_dir.parent().unwrap()))
        }
    }

    fn find_pathrc(&self) -> Option<PathBuf> {
        let dir_path = self.directory.as_path();
        let mut path: PathBuf = dir_path.into();
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
    pub fn print_file_list(&mut self) {
        while let Some(top) = self.found_files.pop() {
            println!("{}", top);
        }
    }
}
