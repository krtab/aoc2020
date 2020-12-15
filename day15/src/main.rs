use std::{
    str::FromStr,
};
use vec_map::VecMap as HashMap;

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let input: Result<Vec<_>, _> = buf.split(',').map(usize::from_str).collect();
    let input = input?;
    let mut history = HashMap::with_capacity(30_000_000);
    let mut turn = 0;
    for i in input {
        history.insert(i, turn);
        turn += 1;
    }
    let mut next_to_say = 0;
    while turn < 30000000-1 {
        // eprintln!("{} : {}", turn, next_to_say);
        let next = history
            .insert(next_to_say, turn)
            .map(|prev_turn| turn - prev_turn)
            .unwrap_or(0);
        next_to_say = next;
        turn += 1;
    }
    // dbg!(history.keys().max().unwrap());
    println!("Answer1: {}", next_to_say);
    Ok(())
}
