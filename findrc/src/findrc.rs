use std::path::{Path, PathBuf};
use std::{env, str};

const PATHRC_FILENAME: &str = ".path-rc";

pub fn get_path() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

pub struct FindRC<'dir> {
    directory: &'dir PathBuf,
    found_files: Vec<String>,
}

impl<'dir> FindRC<'dir> {
    pub fn new(directory: &'dir PathBuf) -> Self {
        Self {
            directory,
            found_files: Vec::new(),
        }
    }

    pub fn search_files(&self) {
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
        let mut path: PathBuf = self.directory.into();
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
    pub fn print_file_list(&self) {
        while let Some(top) = self.found_files.pop() {
            println!("{}", top);
        }
    }
}
