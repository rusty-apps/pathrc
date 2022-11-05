use command::Command;
use command::CommandIgnore;
// use std::fs;
// use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use std::{env, str};

const PATHRC_FILENAME: &str = ".path-rc";

pub fn get_path() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

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
    pub fn consolidate_aliases(&mut self) -> std::io::Result<()> {
        while let Some(top) = self.found_files.pop() {
            // rewrite unalias to format : alias | grep -q git && unalias git
            // let file = fs::File::open(top);
            // let reader = BufReader::new(file);

            for line in my_reader::BufReader::open(top)? {
                let words:Vec = vec!(line.split(" ").collect());
                match Some(words[0]) {
                    Some("unalias") => format("alias | grep -q {} && unalias {}", words[0], words[0]),
                    None => line,
                    }
                }
                // match line.chars().next() {
                //     Some('unalias') =>
                // }

                println!("\n{}", line?.trim());
            }

            // for line in reader.lines() {
            //     println!("\n{}", line);
            // }

            // let contents = fs::read_to_string(top).expect("Should have been able to read the file");
            // println!("\n{contents}");
        }

        Ok(())
    }
}

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
        rc::Rc,
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }
    }

    impl Iterator for BufReader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| {
                    if u == 0 {
                        None
                    } else {
                        Some(Rc::clone(&self.buf))
                    }
                })
                .transpose()
        }
    }
}
