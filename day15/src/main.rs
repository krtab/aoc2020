use std::{num::NonZeroU32 as T, str::FromStr};

fn main() -> anyhow::Result<()> {
    // let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    // let input: Result<Vec<_>, _> = buf.split(',').map(usize::from_str).collect();
    let input = vec![6, 3, 15, 13, 1, 0];
    let mut history = vec![None; 30_000_000];
    let mut turn = 1;
    for i in input {
        unsafe {
            *history.get_unchecked_mut(i) = Some(T::new(turn).unwrap());
        };
        turn += 1;
    }
    let mut next_to_say = 0;
    while turn < 30000000 {
        // eprintln!("{} : {}", turn, next_to_say);
        let tmp = unsafe { history.get_unchecked_mut(next_to_say) };
        let old = std::mem::replace(tmp, unsafe { Some(T::new_unchecked(turn)) });
        let next = old.map(|prev_turn| unsafe { T::new_unchecked(turn - prev_turn.get()) });
        next_to_say = match next {
            None => 0,
            Some(x) => x.get() as usize,
        };
        turn += 1;
    }
    // dbg!(history.keys().max().unwrap());
    println!("Answer1: {}", next_to_say);
    Ok(())
}
