use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};
use std::{env, str};

const PATHRC_FILENAME: &str = ".path-rc";

// struct RcEntry {
//     rc_dir: String,
// }

pub enum PathError {
    MissingPath,
    MissingParent,
    UnknownError,
}

impl PathError {
    fn message(&self) -> &str {
        match self {
            Self::MissingPath => "Missing Path",
            Self::MissingParent => "Missing Parent",
            Self::UnknownError => "Unknown Error",
        }
    }
}

impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message());
    }
}

impl Debug for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message());
    }
}

impl Error for PathError {}

//type Error = PathError;

fn main() {
    let found_files: Vec<&str> = Vec::new();
    //let mut dir = get_path().unwrap();

    // let path = get_path().unwrap();
    // find_pathrc(&path);

    let mut dir = get_path().unwrap();
    dir = find_pathrc(&dir).unwrap();
    let found = dir.to_str().unwrap();
    let dir = next_dir(dir);
    found_files.push(found);
    //let found = PathBuf::new();
    //dir = find_pathrc(&dir);
    //found_files.push()
    // loop {
    //     if dir == None {
    //         break;
    //     }
    // }
    // while dir != None {
    //     found_files.push(found);
    // }
}

fn next_dir((current_dir: &Path) -> Option<PathBuf> {
    unimplemented!()
}

fn get_path() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

fn find_pathrc(starting_directory: &Path) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let file = Path::new(PATHRC_FILENAME);
    let mut dir: PathBuf = PathBuf::new();
    //let mut found_file: PathBuf = PathBuf::new();

    // // fn find_pathrc(starting_directory: &Path) -> Option<(&PathBuf, &PathBuf)> {
    //     let mut path: PathBuf = starting_directory.into();
    //     let file = Path::new(PATHRC_FILENAME);
    //     let mut found_file: PathBuf = PathBuf::new();

    loop {
        path.push(file);
        if path.is_file() {
            dir.push(path.parent().expect("Error getting parent"));
            //found_files.push(path);
            println!("{}", dir.display());
            println!("{}", path.display());
            // found_file = PathBuf::new();
            // found_file = path;
            break Some(dir);
        }

        if !(path.pop() && path.pop()) {
            // remove file && remove parent
            break None;
        }
    }

    //     // loop {
    //     //     path.push(file);
    //     //     if path.is_file() {
    //     //         //found_files.push(path);
    //     //         // println!("{}", path.display());
    //     //         found_file = PathBuf::new();
    //     //         found_file = path;
    //     //         if !(path.pop() && path.pop()) { // remove file && remove parent
    //     //             break;
    //     //     }
    //     //     break;
    //     // }

    //     loop {
    //         path.push(file);
    //         if path.is_dir() {
    //             Some(path);
    //         }

    //         if !(path.pop() && path.pop()) { // remove file && remove parent
    //             break;
    //         }
    //     }
    //     None;
    //unimplemented!();
}
