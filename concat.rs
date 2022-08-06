use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut paths: Vec<_> = fs::read_dir("./logs")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    paths.sort_by_key(|dir| dir.path());
    let mut contents = String::new();

    let header = fs::read_to_string("./partials/header.md").expect("Error!");
    contents.push_str(&header);

    for path in paths {
        let dir = String::from(&path.path().display().to_string());

        let mut v: Vec<&str> = dir.split("/").collect();
        let key = String::from(v.pop().unwrap());

        let md = String::from(&(dir.clone() + "/" + &key + ".md"));

        let img_dir = String::from(&(dir.clone() + "/img"));

        let file_content = fs::read_to_string(md).expect("Error!");

        contents.push_str(&("<div id='".to_owned() + &key + "'></div>\n\n"));

        contents.push_str(&file_content);

        let img_dir_path = Path::new(&img_dir);

        if img_dir_path.is_dir() {
            let img_paths = fs::read_dir(img_dir).unwrap();
            for img_path in img_paths {
                let img = String::from(&img_path.unwrap().path().display().to_string());

                contents.push_str("\n");
                contents.push_str(&("<img src='".to_owned() + &img + "' alt='' />"))
            }
        }

        contents.push_str("<a href='#top'>Back to Top</a>");

        contents.push_str("\n\n");
    }

    let file = File::create("./README.md");

    file.unwrap()
        .write_all(contents.as_bytes())
        .expect("Error!");
}
