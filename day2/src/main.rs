use regex::Regex;
use std::fs::read;
use std::io::{self};

fn main() -> io::Result<()> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)\s*$").unwrap();
    let f = String::from_utf8(read("input.txt")?).unwrap();
    let mut counter1 = 0;
    let mut counter2 = 0;
    for l in f.lines() {
        let cap = re.captures(l).unwrap();
        let lowbound : usize = cap[1].parse().unwrap();
        let highbound : usize = cap[2].parse().unwrap();
        let character = cap[3].chars().next().unwrap();
        let passwd = &cap[4];
        let count = passwd.chars().filter(|&c| c == character).count();
        if lowbound <= count && count <= highbound {
            counter1+=1;
        }
        let chars : Vec<_> = passwd.chars().collect();
        if (chars[lowbound-1] == character)^(chars[highbound-1] == character) {
            counter2 += 1;
        }
    };
    println!("Answer 1: {}\nAnswer 2: {}",counter1,counter2);
    Ok(())
}
