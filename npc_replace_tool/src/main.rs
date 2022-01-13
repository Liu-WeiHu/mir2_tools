use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

fn main() {
    let f2 = File::open(r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\npc.txt"#)
        .unwrap();
    let f2 = BufReader::new(f2);

    let f = File::open(r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\npc2.txt"#)
        .unwrap();
    let f = BufReader::new(f);

    let mut f1 = File::create(
        r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\npcdebug.txt"#,
    )
    .unwrap();

    let path = r#"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Envir\NPCs"#;
    let mut lines = f.lines();
    for line in f2.lines() {
        let line = line.unwrap();
        if let Some(n1) = line.split(',').next() {
            if n1.contains('/') {
                let mut item_path = "\\".to_string();
                let mut i = 0;
                let n1 = n1.split('/').collect::<Vec<_>>();

                if let Some(s2) = lines.next() {
                    let s2 = s2.unwrap();
                    for item in n1.iter() {
                        let item2 = s2.split(',').next().unwrap().split('/').nth(i).unwrap();
                        let item3 = format!("{}{}", item_path, item);
                        let mut source_path = format!("{}{}", path, item3);
                        item_path = format!("{}{}", item_path, item2);
                        let mut dest_path = format!("{}{}", path, item_path);
                        if n1.last().unwrap().to_string() == item.to_string() {
                            source_path += ".txt";
                            dest_path += ".txt";
                        }
                        item_path += "\\";
                        i += 1;
                        if Path::new(&source_path).exists() {
                            f1.write_all(source_path.as_bytes()).unwrap();
                            f1.write_all(b"\n").unwrap();
                            f1.write_all(dest_path.as_bytes()).unwrap();
                            f1.write_all(b"\n").unwrap();
                            std::fs::rename(source_path, dest_path).unwrap();
                        }
                    }
                }
            }
        }
    }
}
