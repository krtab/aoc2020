#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Seat {
    Free,
    Occupied,
}

#[allow(dead_code)]
fn print_layout(layout: &Vec<Vec<Option<Seat>>>) -> String {
    let mut res = String::new();
    for r in layout {
        for s in r {
            let char_repr = match s {
                Some(Seat::Occupied) => '#',
                Some(Seat::Free) => 'L',
                None => '.',
            };
            res.push(char_repr);
        }
        res.push('\n');
    }
    res
}
fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut rows = Vec::new();
    for l in buf.lines() {
        let mut tmp = Vec::new();
        for c in l.chars() {
            tmp.push(match c {
                'L' => Some(Seat::Free),
                '.' => None,
                _ => unreachable!("Unknown character: {}", c),
            })
        }
        rows.push(tmp);
    }
    let nrows = rows.len() as isize;
    let ncols = rows[0].len() as isize;
    let mut previous_state = rows.clone();
    loop {
        let mut new_state = previous_state.clone();
        let mut something_changed = false;
        for i in 0..nrows {
            for j in 0..ncols {
                let mut counter = 0;
                for &(a, b) in &[
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ] {
                    if a < 0 || b < 0 {
                        continue;
                    }
                    match previous_state
                        .get(a as usize)
                        .and_then(|r| r.get(b as usize))
                        .unwrap_or(&None)
                    {
                        Some(Seat::Free) | None => (),
                        Some(Seat::Occupied) => counter += 1,
                    }
                }
                let tmp = match previous_state[i as usize][j as usize] {
                    Some(Seat::Free) if counter == 0 => {
                        something_changed = true;
                        Some(Seat::Occupied)
                    }
                    Some(Seat::Occupied) if counter >= 4 => {
                        something_changed = true;
                        Some(Seat::Free)
                    }
                    x => x,
                };
                new_state[i as usize][j as usize] = tmp;
            }
        }
        // println!("{}",print_layout(&new_state));
        previous_state = new_state;
        if !something_changed {
            break;
        }
    }
    let mut a1 = 0;
    for r in previous_state {
        for s in r {
            if s == Some(Seat::Occupied) {
                a1 += 1;
            }
        }
    }
    println!("Answer1: {}", a1);

    let mut previous_state = rows.clone();
    loop {
        let mut new_state = previous_state.clone();
        let mut something_changed = false;
        for i in 0..nrows {
            for j in 0..ncols {
                let mut counter = 0;
                for &(dir_row, dir_col) in &[
                    (-1, -1),
                    (-1, 00),
                    (-1, 01),
                    (00, -1),
                    (00, 01),
                    (01, -1),
                    (01, 00),
                    (01, 01),
                ] {
                    let mut a = i + dir_row;
                    let mut b = j + dir_col;
                    let seen_seat = loop {
                        if a < 0 || b < 0 {
                            break None;
                        };
                        match previous_state
                        .get(a as usize)
                        .and_then(|r| r.get(b as usize))
                        {
                            None => break None,
                            Some(None) => (),
                            Some(&x) => break x,
                        }
                        a += dir_row;
                        b += dir_col;
                    };
                    match seen_seat
                    {
                        Some(Seat::Free) | None => (),
                        Some(Seat::Occupied) => counter += 1,
                    }
                }
                let tmp = match previous_state[i as usize][j as usize] {
                    Some(Seat::Free) if counter == 0 => {
                        something_changed = true;
                        Some(Seat::Occupied)
                    }
                    Some(Seat::Occupied) if counter >= 5 => {
                        something_changed = true;
                        Some(Seat::Free)
                    }
                    x => x,
                };
                new_state[i as usize][j as usize] = tmp;
            }
        }
        // println!("{}",print_layout(&new_state));
        previous_state = new_state;
        if !something_changed {
            break;
        }
    }
    let mut a2 = 0;
    for r in previous_state {
        for s in r {
            if s == Some(Seat::Occupied) {
                a2 += 1;
            }
        }
    }
    println!("Answer2: {}", a2);
    Ok(())
}
