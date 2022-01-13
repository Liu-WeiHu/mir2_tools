use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
};

use regex::{Captures, Regex};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let file1 = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\ItemInfo Output.csv",
    )?;
    let file1 = BufReader::new(file1);
    let lines1 = file1
        .lines()
        .map(|line| line.unwrap())
        .skip(1)
        .collect::<Vec<_>>();

    let file2 = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\ItemInfo Output2.csv",
    )?;
    let file2 = BufReader::new(file2);
    let lines2 = file2
        .lines()
        .map(|line| line.unwrap())
        .skip(1)
        .collect::<Vec<_>>();

    let mut file4 = File::create(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\itemdebug.txt",
    )?;

    let mut map = HashMap::new();

    for (index, line) in lines1.iter().enumerate() {
        let item1 = line.split(',').next().unwrap();
        let item2 = lines2[index].split(',').next().unwrap();
        map.insert(item1.to_uppercase(), item2);
    }

    //println!("{:?}", map);     //r"(?P<n1>\d+)/(?P<n2>\d+) (?P<name>[a-zA-Z0-9()]+)"
    let re = Regex::new(r"((?P<n1>\d+)/(?P<n2>\d+))\s+(?P<name>[a-zA-Z0-9()'.]+)").unwrap();
    let re2 = Regex::new(r"(\d+/\d+\s.+Q)").unwrap();

    for entry in WalkDir::new(r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Envir\Drops")
        .min_depth(1)
    {
        let line = entry.unwrap();
        if File::open(line.path()).is_err() {
            continue;
        }
        let mut file3 = File::open(line.path())?;
        file4.write_all(line.path().to_str().unwrap().as_bytes())?;
        file4.write_all(b"\n")?;
        let mut contents = String::new();
        file3.read_to_string(&mut contents).unwrap();
        //println!("{}", contents);

        contents = re
            .replace_all(&contents, |cap: &Captures| {
                let ss = cap.name("name").unwrap().as_str();
                let sss = ss.to_uppercase();
                //println!("{}", sss);
                format!(
                    "{}/{} {}",
                    cap.name("n1").unwrap().as_str(),
                    cap.name("n2").unwrap().as_str(),
                    map.get(&sss).unwrap_or(&ss),
                )
            })
            .to_string();
        contents = re2.replace_all(&contents, "").to_string();
        //println!("{}", contents);
        fs::remove_file(line.path())?;

        let mut file3 = File::create(line.path())?;
        file3.write_all(contents.as_bytes()).unwrap();
    }

    Ok(())
}
