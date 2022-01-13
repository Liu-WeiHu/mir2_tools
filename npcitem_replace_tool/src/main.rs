use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
};

use regex::{Captures, Regex};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let file1 = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server1\Exports\ItemInfo Output.csv",
    )?;
    let file1 = BufReader::new(file1);
    let lines1 = file1
        .lines()
        .map(|line| line.unwrap())
        .skip(1)
        .collect::<Vec<_>>();

    let file2 = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server1\Exports\ItemInfo Output2.csv",
    )?;
    let file2 = BufReader::new(file2);
    let lines2 = file2
        .lines()
        .map(|line| line.unwrap())
        .skip(1)
        .collect::<Vec<_>>();

    // let mut file4 = File::create(r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server1\Exports\npcitemdebug.txt")?;

    let mut map = HashMap::new();

    for (index, line) in lines1.iter().enumerate() {
        let item1 = line.split(',').next().unwrap();
        let item2 = lines2[index].split(',').next().unwrap();
        map.insert(item1.to_uppercase(), item2);
    }

    /*let map_str = serde_json::to_string(&map).unwrap();
    file4.write_all(map_str.as_bytes()).unwrap();*/

    //println!("{:?}", map);     //r"(?P<n1>\d+)/(?P<n2>\d+) (?P<name>[a-zA-Z0-9()]+)"
    let re = Regex::new(r"(\[Trade\]\s\n)([a-zA-Z0-9()\s]+)(\[?)").unwrap();

    for entry in WalkDir::new(r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server1\Envir\NPCs")
        .min_depth(1)
    {
        let line = entry.unwrap();
        if File::open(line.path()).is_err() {
            continue;
        }
        let mut file3 = File::open(line.path())?;
        let mut contents = String::new();
        file3.read_to_string(&mut contents).unwrap();
        contents = re
            .replace(&contents, |cap: &Captures| {
                format!(
                    "{}{}\r\n{}",
                    cap.get(1).unwrap().as_str(),
                    cap.get(2)
                        .unwrap()
                        .as_str()
                        .split('\n')
                        .map(|s| map
                            .get(&s.trim().to_uppercase())
                            .copied()
                            .map(ToString::to_string)
                            .unwrap_or_else(|| {
                                if s.contains(' ') {
                                    let mut sp = s.split(' ');
                                    format!(
                                        "{} {}",
                                        map.get(&sp.next().unwrap().to_uppercase()).unwrap(),
                                        sp.next().unwrap()
                                    )
                                } else {
                                    s.to_owned()
                                }
                            }))
                        .collect::<Vec<_>>()
                        .join("\n"),
                    cap.get(3).unwrap().as_str(),
                )
            })
            .to_string();
        fs::remove_file(line.path())?;

        let mut file3 = File::create(line.path())?;
        file3.write_all(contents.as_bytes()).unwrap();
    }

    Ok(())
}
