
use std::env;
use std::path::{Path, PathBuf};

const PATHRC_FILENAME: &str = ".path-rc";


fn main() {
    
    let mut rc_paths: Vec<String> = Vec::new();

    let mut path = get_path().unwrap();
    //let mut rc_path = find_pathrc(&path);

    while let Some(path_buf) = find_pathrc(&path) {
        println!("{}", path_buf.display());
        //rc_paths.push(rc_path.to_path());
        //let rc_dir = path_buf.to_path();
        let path = path_buf.parent();
    }

    // while let Some(num) = my_picks.pop() {
    //     println!("{}", num);
    // }
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
            //println!("{}", path.display());
            let path_dir = path.parent();
            let buf = PathBuf::from(path_dir.unwrap());
            break Some(buf);
        }

        if !(path.pop() && path.pop()) { // remove file && remove parent
            break None;
        }
    }
}