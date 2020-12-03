use std::fs::read;

#[derive(Debug)]
enum Square {
    Tree,
    Empty,
}

fn main() -> anyhow::Result<()> {
    let f = String::from_utf8(read("input.txt")?)?;
    let mut map = Vec::new();
    for l in f.lines() {
        let mut tmp = Vec::new();
        for c in l.chars() {
            tmp.push(match c {
                '.' => Square::Empty,
                '#' => Square::Tree,
                c => panic!("Encountered unknown char {:?}", c),
            });
        }
        map.push(tmp);
    }
    let ncols = map.first().unwrap().len();
    {
        let mut ridx = 0;
        let mut colidx = 0;
        let mut counter = 0;
        loop {
            colidx %= ncols;
            match map.get(ridx) {
                None => break,
                Some(row) => match row[colidx] {
                    Square::Empty => {}
                    Square::Tree => counter += 1,
                },
            }
            ridx += 1;
            colidx += 3;
        }
        println!("There was {} trees.", counter);
    }
    let mut res : u64 = 1;
    for (colstep, rowstep) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut ridx = 0;
        let mut colidx = 0;
        let mut counter = 0;
        loop {
            colidx %= ncols;
            match map.get(ridx) {
                None => break,
                Some(row) => match row[colidx] {
                    Square::Empty => {}
                    Square::Tree => counter += 1,
                },
            }
            ridx += rowstep;
            colidx += colstep;
        }
        res *= dbg!(counter);
    }
    println!("Answer 2: {}", res);
    Ok(())
}
