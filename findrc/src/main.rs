
use std::env;
use std::path::{Path, PathBuf};

const PATHRC_FILENAME: &str = ".path-rc";

fn main() {

    let path = get_path().unwrap();
    find_pathrc(&path);
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
            println!("{}", path.display());
            break Some(path);
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break None;
        }
    }
}