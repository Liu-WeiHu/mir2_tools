use std::io::Write;
use std::{
    collections::HashMap,
    fs,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use regex::{Captures, Regex};

fn main() -> std::io::Result<()> {
    let file1 = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\MonsterInfo Output.csv",
    )?;
    let file1 = BufReader::new(file1);
    let lines1 = file1
        .lines()
        .map(|line| line.unwrap())
        .skip(1)
        .collect::<Vec<_>>();

    let file2 = File::open(
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Exports\MonsterInfo Output2.csv",
    )?;
    let file2 = BufReader::new(file2);
    let lines2 = file2
        .lines()
        .map(|line| line.unwrap())
        .skip(1)
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    for (index, line) in lines1.iter().enumerate() {
        let item1 = line.split(',').next().unwrap();
        let item2 = lines2[index].split(',').next().unwrap();
        map.insert(item1.to_uppercase(), item2);
    }

    let mut map2 = HashMap::new();
    map2.insert(
        1,
        r"欢迎光临。
从现在开始，许多怪物将出现在这个房间。
通过打败怪物来测试你的能力。
总共有20个阶段。
我想知道你是否能打败他们……
你会开始挑战吗?",
    );
    map2.insert(
        2,
        r"你还没有打败他们。
在打败怪物之前，
您无法进入下一阶段。",
    );
    map2.insert(
        3,
        r"你已经把他们都打败了。
但还有更多的事情要做。
你要继续挑战吗？",
    );
    map2.insert(
        4,
        r"哇，你把他们都打败了!!
哦……怎么会有人如此强大…
见到你我很荣幸。
你是真正的英雄，当危机到来时，你可以拯救大陆。
我会带着奖品送你回村子。",
    );

    let paths = vec![
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Envir\NPCs\比奇省\无限挑战地图\开始30-EM001.txt",
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Envir\NPCs\比奇省\无限挑战地图\开始40-EM002.txt",
        r"C:\Users\liu\Documents\Github\mir2\mir2\Build\Server\Envir\NPCs\比奇省\无限挑战地图\开始41-EM003.txt",
    ];

    let mut content = String::new();

    let re = Regex::new(r"(MonGen\s)(.+)(\s\d)").unwrap();
    let re1 = Regex::new(r"(Welcome[\.a-zA-Z0-9\s',!]*start the challenge\?)").unwrap();
    let re2 = Regex::new(r"(You ha[\.a-zA-Z0-9\s',!]*stage\.)").unwrap();
    let re3 = Regex::new(r"(You de[\.a-zA-Z0-9\s',!]*continue the challenge\?)").unwrap();
    let re4 = Regex::new(r"(Wow[\.a-zA-Z0-9\s',!]*prize\.)").unwrap();

    for p in paths {
        let mut file = File::open(p).unwrap();
        file.read_to_string(&mut content).unwrap();

        content = content.replace(
            "You have already completed this quest!",
            "你已经完成了这个任务!",
        );
        content = content.replace("I'm ready.", "我准备好了.");
        content = content.replace("Proceed.", "继续");
        content = content.replace("I've had enough. Let me go.", "我受够了，让我走。");
        content = content.replace("MOVE 0 355 237", "MOVE n0 328 265");
        content = content.replace("BenedictionOil", "祝福油");

        let content = re
            .replace_all(&content, |cap: &Captures| {
                let mon = cap.get(1).unwrap().as_str();
                map.get(&mon.trim().to_uppercase())
                    .copied()
                    .unwrap_or(mon)
                    .to_string();
                format!(
                    "{}{}{}",
                    cap.get(1).unwrap().as_str(),
                    map.get(&cap.get(2).unwrap().as_str().trim().to_uppercase())
                        .copied()
                        .unwrap_or(mon),
                    cap.get(3).unwrap().as_str(),
                )
            })
            .to_string();

        /*let cap = re1.captures(&content).unwrap();
        println!("{}", cap.get(1).unwrap().as_str());*/
        let content = re1
            .replace_all(&content, map2.get(&1).copied().unwrap().to_string())
            .to_string();

        let content = re2
            .replace_all(&content, map2.get(&2).copied().unwrap().to_string())
            .to_string();

        let content = re3
            .replace_all(&content, map2.get(&3).copied().unwrap().to_string())
            .to_string();

        let content = re4
            .replace_all(&content, map2.get(&4).copied().unwrap().to_string())
            .to_string();

        fs::remove_file(p)?;
        let mut file3 = File::create(p)?;
        file3.write_all(content.as_bytes()).unwrap();
    }

    Ok(())
}
