mod main;

// use std::env;
// use std::path::{Path, PathBuf};
use std::path::Path;

const PATHRC_FILENAME: &str = ".path-rc";

#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     assert_eq!(2 + 2, 4);
    // }

    #[test]
    

    fn test_get_path() {
        //let dir = Path::new("path-rc-file/test");
        // assert!(env::set_current_dir(&dir).is_ok());

        let path = get_path().unwrap();
        println!("Successfully changed working directory to {}!", path.display());
    }



    // #[test]
    // fn find_dot_path_rc_file() -> Result<(), Box<dyn std::error::Error>> {

    //     let dir = Path::new("path-rc-file/test");
    //     assert!(env::set_current_dir(&dir).is_ok());
    //     println!("Successfully changed working directory to {}!", dir.display());
        
    //     let mut file = NamedTempFile::new()?;
    //     writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    //     let mut cmd = Command::cargo_bin("grrs")?;
    //     cmd.arg("test").arg(file.path());
    //     cmd.assert()
    //         .success()
    //         .stdout(predicate::str::contains("test\nAnother test"));

    //     Ok(())
    // }
}
