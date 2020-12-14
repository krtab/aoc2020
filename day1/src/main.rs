use std::cmp::{Ord,Ordering};
use std::fs;
use std::str::{self,FromStr};
use anyhow;
use std::io::{stdout,BufWriter,Write};

fn main() -> anyhow::Result<()> {
    let stdout = stdout();
    let mut handle = BufWriter::new(stdout.lock());
    let buf = fs::read("input.txt")?;
    let s = str::from_utf8(&buf)?;
    let input : Result<Vec<i64>,<i64 as FromStr>::Err> = s.lines().map(|x| (x.parse())).collect();
    let mut input = input?;
    input.sort_unstable();
    let mut iter = input.iter();
    let mut left = iter.next();
    let mut right = iter.next_back();
    while let Some((l,r)) = left.and_then(|l| right.map(|r| (l,r))) {
        use Ordering::*;
        let ord = Ord::cmp( &(l+r), &2020);
        match ord {
            Less => left = iter.next(),
            Greater => right = iter.next_back(),
            Equal => {
                writeln!(handle,"Result: {}x{} = {}", l, r, l*r)?;
                break
            }
        }
    }
    for i1 in &input {
        for i2 in &input {
            for i3 in &input {
                if i1 + i2 + i3 == 2020 {
                    writeln!(handle,"Result: {} x {} x {} = {}", i1, i2, i3, i1*i2*i3)?;
                }
            }
        }
    }
    Ok(())
}
