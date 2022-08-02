use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut paths: Vec<_> = fs::read_dir("./log")
                                .unwrap()
                                .map(|r| r.unwrap())
                                .collect();
    paths.sort_by_key(|dir| dir.path());
    let mut contents = String::new();
    contents.push_str("# Renovation 2022\n\n");

    for path in paths {
        let file_content = fs::read_to_string(&path.path())
        .expect("Error!");
        contents.push_str(&file_content);
        contents.push_str("\n\n");
    }

    let file = File::create("./README.md");

    file.unwrap().write_all(contents.as_bytes()).expect("Error!");
}
