use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() {
    let f = File::open(
        r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\MapInfoExport.txt"#,
    )
    .unwrap();
    let f = BufReader::new(f);

    let mut map = HashMap::new();
    for line in f.lines() {
        let line = line.unwrap();
        if line.starts_with('[') {
            let dest = line.split(']').next().unwrap().trim_start_matches('[');
            let mut sp = dest.split(' ');
            map.insert(
                sp.next().unwrap().to_ascii_uppercase(),
                sp.next().unwrap().to_string(),
            );
        }
    }

    let f = File::open(
        r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\MapInfoExport.txt"#,
    )
    .unwrap();
    let f = BufReader::new(f);

    let mut file = File::create(
        r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\MapInfoExport2.txt"#,
    )
    .unwrap();
    for line in f.lines() {
        let line = line.unwrap();
        if line.starts_with('[') {
            let mut sp1 = line.split(']');
            if let Some(dest) = sp1.next() {
                let mut sp = dest.trim_start_matches('[').split(' ');
                if let Some(one) = sp.next() {
                    if let Some(val) = map.get(&one.to_ascii_uppercase()) {
                        if let Some(right) = sp1.next() {
                            let s = format!("[{} {}]{}\n", one, val, right);
                            file.write_all(s.as_bytes()).unwrap();
                            continue;
                        }
                    }
                }
            }
        }
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
