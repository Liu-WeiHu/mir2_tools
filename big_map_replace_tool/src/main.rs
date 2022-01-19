use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut f = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server1\Exports\MapInfoExport2.txt",
    )
    .unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let re = regex::Regex::new(r"(.+MINIMAP\()(.+)(\)\sBIGMAP\()(.+)(\)\sMAPLIGHT)").unwrap();

    let s = re
        .replace_all(&content, |cap: &regex::Captures| {
            format!(
                "{}{}{}{}{}",
                cap.get(1).unwrap().as_str(),
                cap.get(2).unwrap().as_str(),
                cap.get(3).unwrap().as_str(),
                cap.get(2).unwrap().as_str(),
                cap.get(5).unwrap().as_str(),
            )
        })
        .to_string();

    let mut f2 = File::create(r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server1\Exports\MapInfoExport3.txt").unwrap();
    f2.write_all(s.as_bytes()).unwrap();
}
