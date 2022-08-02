use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let paths = fs::read_dir("./log").unwrap();
    let mut contents = String::new();
    contents.push_str("# Renovation 2022\n\n");

    for path in paths {
        let file_content = fs::read_to_string(&path.unwrap().path())
        .expect("Error!");
        contents.push_str(&file_content)

    }

    let file = File::create("./README.md");

    file.unwrap().write_all(contents.as_bytes());
}