use std::fs::OpenOptions;
use std::os::windows::prelude::FileExt;
use std::{fs::File, io::Read};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    let re = regex::Regex::new("(.+)[a-zA-Z]{1}.txt").unwrap();
    for entry in WalkDir::new(r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Envir\Drops")
        .min_depth(1)
    {
        let line = entry?;
        if File::open(line.path()).is_err() {
            continue;
        }
        let file_name = line.file_name().to_str().unwrap();
        if file_name.ends_with("0.txt") {
            let mut file = options.read(true).write(true).open(line.path())?;
            let mut _v = Vec::new();
            let len = file.read_to_end(&mut _v)?;
            let content = "\n\n;Scroll
1/2 1元宝兑换券 1";
            file.seek_write(content.as_bytes(), len as u64)?;
        } else if re.is_match(file_name) {
            let mut file = options.read(true).write(true).open(line.path())?;
            let mut _v = Vec::new();
            let len = file.read_to_end(&mut _v)?;
            let content = "\n\n;Scroll
1/150 5元宝兑换券 1";
            file.seek_write(content.as_bytes(), len as u64)?;
        }
    }
    Ok(())
}
