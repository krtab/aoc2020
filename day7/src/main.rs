use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() -> anyhow::Result<()> {
    let parent_regex = Regex::new(r"^(?P<color>\w+ \w+) bags")?;
    let child_regex = Regex::new(r"(?P<number>\d+) (?P<color>\w+ \w+) bag")?;
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut contains = HashMap::new();
    let mut contained = HashMap::new();
    for l in buf.lines() {
        let parent = parent_regex
            .captures(l)
            .unwrap()
            .name("color")
            .unwrap()
            .as_str();
        let children = child_regex.captures_iter(l).map(|cap| {
            (
                cap.name("number").unwrap().as_str().parse().unwrap(),
                cap.name("color").unwrap().as_str(),
            )
        });
        for (n, c) in children {
            let entry = contains.entry(parent).or_insert_with(Vec::new);
            entry.push((n, c));
            let entry = contained.entry(c).or_insert_with(Vec::new);
            entry.push(parent);
        }
    }
    let mut visited = HashSet::new();
    let mut queue = vec!["shiny gold"];
    let mut first_iter = true;
    while let Some(col) = queue.pop() {
        if first_iter || visited.insert(col) {
            for p in contained.get(col).unwrap_or(&Vec::new()) {
                queue.push(p);
            }
            first_iter = false;
        }
    }
    println!("Answer 1: {}", visited.len());
    let mut queue = vec![(1,"shiny gold")];
    let mut counter : u64 = 0;
    while let Some((multiplier,col)) = queue.pop() {
        counter += multiplier;
        for (n,c) in contains.get(col).unwrap_or(&Vec::new()) {
            queue.push((multiplier*n,c))
        }
    }
    println!("Answer 2: {}", counter-1);
    Ok(())
}
