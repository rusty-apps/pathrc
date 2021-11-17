use findrc::FindRC;

fn main() {
    let dir = findrc::get_path().unwrap();
    let mut find_rc = FindRC::new(dir);
    find_rc.search_files();
    find_rc.print_file_list();
}
