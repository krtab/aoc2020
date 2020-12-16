mod input;

use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() -> anyhow::Result<()> {
    let (nearby_tickets, my_ticket) = input::init();
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let regex = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$")?;
    let mut rules = HashMap::new();
    for l in buf.lines() {
        let cap = regex.captures(l).unwrap();
        rules.insert(
            cap.get(1).unwrap().as_str(),
            (
                RangeInclusive::new(
                    cap[2].parse::<u64>().unwrap(),
                    cap[3].parse::<u64>().unwrap(),
                ),
                RangeInclusive::new(
                    cap[4].parse::<u64>().unwrap(),
                    cap[5].parse::<u64>().unwrap(),
                ),
            ),
        );
    }
    let error_rate: u64 = nearby_tickets
        .iter()
        .flatten()
        .filter(|&t| {
            !rules
                .values()
                .any(|(r1, r2)| r1.contains(t) || r2.contains(t))
        })
        .sum();
    println!("Answer1: {}", error_rate);
    let good_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|&fields| {
            fields.iter().all(|f| {
                rules
                    .values()
                    .any(|(r1, r2)| r1.contains(f) || r2.contains(f))
            })
        })
        .collect();
    let mut applyable_rules: Vec<Vec<_>> = vec![rules.iter().collect(); 20];
    for t in good_tickets {
        for (app_rules, f) in applyable_rules.iter_mut().zip(t) {
            app_rules.retain(|&(_, (r1, r2))| r1.contains(f) || r2.contains(f))
        }
    }
    let mut applyable_rule_names: Vec<(usize, Vec<&str>)> = applyable_rules
        .iter()
        .map(|x| x.iter().map(|x| *x.0).collect())
        .enumerate()
        .collect();
    applyable_rule_names.sort_unstable_by_key(|(_, v)| v.len());
    let mut field_names = HashMap::new();
    for (field_idx, rule_names) in applyable_rule_names {
        let &name = rule_names
            .iter()
            .find(|&n| !field_names.values().any(|nprime| n == nprime))
            .unwrap();
        field_names.insert(field_idx, name);
    }
    let mut res = 1;
    for (&field_idx, &rule_name) in field_names.iter() {
        if rule_name.starts_with("departure") {
            res *= my_ticket[field_idx];
        }
    }
    println!("Answer2: {}", res);
    Ok(())
}
