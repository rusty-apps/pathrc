use std::fs;
use std::path::{Path, PathBuf};

const PATHRC_DEPTH_DEFAULT: u16 = 1000;

pub struct PathRC {
    directory: PathBuf,
    found_files: Vec<String>,
}

impl PathRC {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            found_files: Vec::new(),
        }
    }

    pub fn search_files(&mut self, file_name: String) {
        let mut depth = envmnt::get_u16("PATHRC_DEPTH", PATHRC_DEPTH_DEFAULT);
        while let Some(path_buf) = Self::find_pathrc(&self, &file_name) {
            let p_dir = Self::next_dir(path_buf.as_path());
            if p_dir == None || depth == 0 {
                break;
            } else {
                self.directory = p_dir.unwrap();
                depth = depth - 1
            }
            let p_file = path_buf.to_string_lossy() + "/" + file_name.as_str();
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

    fn find_pathrc(&self, file_name: &String) -> Option<PathBuf> {
        let dir_path = self.directory.as_path();
        let mut path: PathBuf = dir_path.into();
        let file = Path::new(file_name);
        loop {
            path.push(file);
            if path.is_file() {
                // if args list then print file name
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

    pub fn print_file_names(&mut self) {
        println!("\npathrc file processing order:\n");
        while let Some(top) = self.found_files.pop() {
            println!("{}", top.to_string());
        }
    }

    pub fn print_files_contents(&mut self) {
        while let Some(top) = self.found_files.pop() {
            let filename = top.to_string();
            let err_message = format!("Unable to read {}", filename);
            println!("# File: {}", filename);
            let contents = fs::read_to_string(filename).expect(err_message.as_str());
            print!("{contents}");
        }
    }
}

