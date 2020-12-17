use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cube {
    Active,
    Inactive,
}

type PocketDimension = VecDeque<VecDeque<VecDeque<VecDeque<Cube>>>>;

#[allow(dead_code)]
fn print_layout(layout: &PocketDimension) -> String {
    use std::fmt::Write;
    let n_stack = (layout.len() - 1) / 2;
    let mut res = String::new();
    let mut z = -(n_stack as isize);
    let mut w = -(n_stack as isize);
    for hyper_plane in layout {
        writeln!(res, "w = {}", w).unwrap();
        for plane in hyper_plane {
            writeln!(res, "z = {}", z).unwrap();
            for r in plane {
                for s in r {
                    let char_repr = match s {
                        Cube::Active => '#',
                        Cube::Inactive => '.',
                    };
                    res.push(char_repr);
                }
                res.push('\n');
            }
            println!();
            z += 1;
        }
        w += 1;
    }
    res
}

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut hyper_planes: PocketDimension = VecDeque::new();
    hyper_planes.push_back(VecDeque::new());
    hyper_planes[0].push_back(VecDeque::new());
    for l in buf.lines() {
        let mut tmp = VecDeque::new();
        for c in l.chars() {
            tmp.push_back(match c {
                '#' => Cube::Active,
                '.' => Cube::Inactive,
                _ => unreachable!("Unknown character: {}", c),
            })
        }
        hyper_planes[0][0].push_back(tmp);
    }
    let mut nrows = hyper_planes[0][0].len();
    let mut ncols = hyper_planes[0][0][0].len();
    let mut previous_state: PocketDimension = hyper_planes.clone();
    for niter in 0..6 {
        let empty_line = VecDeque::from(vec![Cube::Inactive; ncols + 2]);
        let empty_plane = VecDeque::from(vec![empty_line.clone(); nrows + 2]);
        let empty_hyper_plane = VecDeque::from(vec![empty_plane.clone(); 2 * niter + 3]);
        for hyper_plane in &mut previous_state {
            for plane in hyper_plane.iter_mut() {
                for row in plane.iter_mut() {
                    row.push_back(Cube::Inactive);
                    row.push_front(Cube::Inactive);
                }
                plane.push_back(empty_line.clone());
                plane.push_front(empty_line.clone());
            }
            hyper_plane.push_back(empty_plane.clone());
            hyper_plane.push_front(empty_plane.clone());
        }
        previous_state.push_back(empty_hyper_plane.clone());
        previous_state.push_front(empty_hyper_plane);
        nrows += 2;
        ncols += 2;
        let mut new_state = previous_state.clone();
        for h in 0..previous_state.len() as isize {
            for i in 0..previous_state.len() as isize {
                for j in 0..nrows as isize {
                    for k in 0..ncols as isize {
                        let mut counter = 0;
                        for &z in &[h - 1, h, h + 1] {
                            for &a in &[i - 1, i, i + 1] {
                                for &b in &[j - 1, j, j + 1] {
                                    for &c in &[k - 1, k, k + 1] {
                                        if a < 0
                                            || b < 0
                                            || c < 0
                                            || z < 0
                                            || (a == i && b == j && c == k && z == h)
                                        {
                                            continue;
                                        }
                                        match previous_state
                                            .get(z as usize)
                                            .and_then(|hp| hp.get(a as usize))
                                            .and_then(|p| p.get(b as usize))
                                            .and_then(|r| r.get(c as usize))
                                        {
                                            Some(Cube::Inactive) | None => (),
                                            Some(Cube::Active) => counter += 1,
                                        }
                                    }
                                    let tmp =
                                        match previous_state[h as usize][i as usize][j as usize][k as usize] {
                                            Cube::Active => {
                                                if counter == 2 || counter == 3 {
                                                    Cube::Active
                                                } else {
                                                    Cube::Inactive
                                                }
                                            }
                                            Cube::Inactive => {
                                                if counter == 3 {
                                                    Cube::Active
                                                } else {
                                                    Cube::Inactive
                                                }
                                            }
                                        };
                                    new_state[h as usize][i as usize][j as usize][k as usize] = tmp;
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("{}",print_layout(&new_state));
        previous_state = new_state;
    }
    let a1 = previous_state
        .iter()
        .flatten()
        .flatten()
        .flatten()
        .filter(|&&c| c == Cube::Active)
        .count();
    println!("Answer1: {}", a1);

    Ok(())
}
