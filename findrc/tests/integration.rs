use findrc::FindRC;

#[test]
fn starting_directory_correct() {
    let expected_path: &str = "path-rc/findrc";
    let path = findrc::get_path()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        .to_owned();
    let path_str: &str = &*path;
    let offset = path_str.len() - 14;
    let last_chars = &path_str[offset..];

    assert_eq!(
        expected_path, last_chars,
        "Test that path ending: {} matches path ending: {}",
        expected_path, last_chars
    );
}

#[test]
fn find_rc_file() {
    let dir = findrc::get_path().unwrap();
    let mut find_rc = FindRC::new(dir);
    find_rc.search_files();
    let found_files = find_rc.found_files();
    let path = &found_files[0];

    let expected_path: &str = ".path-rc";
    let path_str: &str = &*path;
    let offset = path_str.len() - 8;
    let last_chars = &path_str[offset..];

    assert_eq!(
        expected_path, last_chars,
        "Test that we have the .path-rc file: {} matches path ending: {}",
        expected_path, last_chars
    );
}
