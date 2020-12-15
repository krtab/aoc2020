type T = u32;

fn main() {
    // let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    // let input: Result<Vec<_>, _> = buf.split(',').map(usize::from_str).collect();

    let input = vec![6, 3, 15, 13, 1, 0];
    let mut history = vec![T::MAX; 30_000_000];
    let mut turn = 1;
    for i in input {
        unsafe {
            *history.get_unchecked_mut(i) = turn;
        };
        turn += 1;
    }
    let mut next_to_say = 0;
    while turn < 30_000_000 {
        let tmp = unsafe { history.get_unchecked_mut(next_to_say) };
        let old = std::mem::replace(tmp, turn);
        next_to_say = turn.saturating_sub(old) as usize;
        turn += 1;
    }
    // dbg!(history.keys().max().unwrap());
    println!("Answer1: {}", next_to_say);
}
