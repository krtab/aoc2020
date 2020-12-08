use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYECOLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() -> anyhow::Result<()> {
    let regex_parse = Regex::new(r"(\S+):(\S+)")?;
    let regex_height = Regex::new(r"^(\d+)((cm)|(in))$")?;
    let regex_hexacolor = Regex::new(r"^#[0-9a-f]{6}$")?;
    let regex_passport_id = Regex::new(r"^\d{9}$")?;
    let validation: [(&str, Box<dyn Fn(&str) -> bool>); 7] = [
        //yr (Birth Year) - four digits; at least 1920 and at most 2002.
        (
            "byr",
            Box::new(|s: &str| {
                let year = s.parse().unwrap();
                (1920 <= year) & (year <= 2002)
            }),
        ),
        //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        (
            "iyr",
            Box::new(|s: &str| {
                let year = s.parse().unwrap();
                (2010 <= year) & (year <= 2020)
            }),
        ),
        //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        (
            "eyr",
            Box::new(|s: &str| {
                let year = s.parse().unwrap();
                (2020 <= year) & (year <= 2030)
            }),
        ),
        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        (
            "hgt",
            Box::new(|s| {
                let cap = regex_height.captures(s);
                match cap {
                    None => false,
                    Some(cap) => {
                        let size = cap[1].parse().unwrap();
                        match &cap[2] {
                            "cm" => (150 <= size) & (size <= 193),
                            "in" => (59 <= size) & (size <= 76),
                            _ => unreachable!(),
                        }
                    }
                }
            }),
        ),
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        ("hcl", Box::new(|s| regex_hexacolor.is_match(s))),
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        ("ecl", Box::new(|s| EYECOLORS.contains(&s))),
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        ("pid", Box::new(|s| regex_passport_id.is_match(s))),
    ];
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut counter1 = 0;
    let mut counter2 = 0;
    for doc in buf.split("\n\n") {
        let map: HashMap<_, _> = regex_parse
            .captures_iter(doc)
            .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
            .collect();
        if FIELDS.iter().all(|f| map.contains_key(f)) {
            counter1 += 1;
            // let foo : Vec<_> = validation.iter().map(|(key, f)| (key,f(map.get(key).unwrap()))).collect();
            // dbg!(foo);
            if validation.iter().all(|(key, f)| f(map.get(key).unwrap())) {
                counter2 += 1;
            }
        }
    }
    println!("Answer 1: {}\nAnswer 2: {}", counter1, counter2);
    Ok(())
}
