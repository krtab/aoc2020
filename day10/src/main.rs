use std::str::FromStr;
use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut input: Vec<_> = buf.lines().map(u64::from_str).map(Result::unwrap).collect();
    input.sort_unstable();
    let mut previous = 0;
    let mut diffs = vec![0,0,0,1];
    for &i in &input {
        diffs[(i - previous) as usize] += 1;
        previous = i;
    }
    println!("Part1: distribution: {:?}, answer: {}",&diffs, diffs[1]*diffs[3]);
    let &max = input.iter().max().unwrap();
    let input : HashSet<_> = input.iter().collect();
    let mut m3 : u64 = 1;
    let mut m2 = if input.contains(&1) {m3} else {0};
    let mut m1 = if input.contains(&2) {m3 + m2} else {0};
    for i in 3..=max {
        let tmp = if input.contains(&i) {
            m3 + m2 + m1
        } else {
            0
        };
        m3 = m2;
        m2 = m1;
        m1 = tmp;
    }
    println!("Answer2: {}", m1);
    Ok(())
}
