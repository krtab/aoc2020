fn main() -> anyhow::Result<()> {
    let mut state: Vec<u8> = Vec::from({
        // example
        // [3, 8, 9, 1, 2, 5, 4, 6, 7]
        // input
        [2, 1, 9, 3, 4, 7, 8, 6, 5]
    });
    let mut buf = state.clone();
    buf.clear();
    for _ in 0..100 {
        if let [current_cup, picked1, picked2, picked3, rest @ ..] = &state[..] {
            let (_dest_cup, dest_index) = {
                let mut destination_cup = *current_cup;
                loop {
                    destination_cup = (destination_cup - 1) % 9;
                    if destination_cup == 0 {
                        destination_cup = 9;
                    }
                    match rest.iter().position(|&x| x == destination_cup) {
                        Some(idx) => break (destination_cup, idx),
                        None => (),
                    }
                }
            };
            let (left, right) = rest.split_at(dest_index + 1);
            buf.extend_from_slice(left);
            buf.extend_from_slice(&[*picked1, *picked2, *picked3]);
            buf.extend_from_slice(right);
            buf.push(*current_cup);
            std::mem::swap(&mut state, &mut buf);
            buf.clear();
        }
    }
    let one_idx = state.iter().position(|&x| x == 1).unwrap();
    state.rotate_left(one_idx);
    print!("Answer1: ");
    for &i in &state[1..] {
        print!("{}", i);
    }
    println!();
    let mut succ = vec![2; 1_000_001];
    let iter = ({
        // example
        // [3, 8, 9, 1, 2, 5, 4, 6, 7]
        // input
        [2, 1, 9, 3, 4, 7, 8, 6, 5]
    })
    .iter()
    .copied()
    .chain(10..=1_000_000);
    for (x, next) in iter.clone().zip(iter.skip(1)) {
        succ[x] = next;
    }
    let mut current = 2;
    for _ in 0..10_000_000 {
        let p1 = succ[current];
        let p2 = succ[p1];
        let p3 = succ[p2];
        let first_rest = succ[p3];
        let dest_cup = {
            let mut destination_cup = current;
            loop {
                destination_cup = destination_cup - 1;
                if destination_cup == 0 {
                    destination_cup = 1_000_000 ;
                }
                if (destination_cup != p1) && (destination_cup != p2) && (destination_cup != p3) {
                    break destination_cup;
                }
            }
        };
        succ[current] = first_rest;
        let prev_succ_dest_cup = succ[dest_cup];
        succ[dest_cup] = p1;
        succ[p3] = prev_succ_dest_cup;
        current = succ[current];
    }
    let a2 = succ[1] * succ[succ[1]];
    println!("Answer2: {}", a2);
    Ok(())
}
