use std::cmp::Ordering;
use std::collections::VecDeque;
use std::str::FromStr;

const CYCLE_SIZE: usize = 25;

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let entries: Vec<_> = buf.lines().map(|s| u64::from_str(s).unwrap()).collect();
    let mut ringbuf = VecDeque::with_capacity(CYCLE_SIZE);
    ringbuf.extend(&entries[..CYCLE_SIZE]);
    let answer1 : u64 = {
        let mut iter =  entries[CYCLE_SIZE..].iter();
        'entries: loop {
            let &e = iter.next().unwrap();
            let mut tmp: Vec<_> = ringbuf.iter().copied().collect();
            tmp.sort_unstable();
            let mut iter = tmp.iter();
            let mut left = iter.next();
            let mut right = iter.next_back();
            while let Some((l, r)) = left.and_then(|l| right.map(|r| (l, r))) {
                use Ordering::*;
                let ord = Ord::cmp(&(l + r), &e);
                match ord {
                    Less => left = iter.next(),
                    Greater => right = iter.next_back(),
                    Equal => {
                        ringbuf.pop_front();
                        ringbuf.push_back(e);
                        continue 'entries;
                    }
                }
            }
            break e;
        }
    };
    println!("Answer1: {}", answer1);
    ringbuf.clear();
    let mut iter = entries.iter();
    let mut sum = 0;
    loop {
        use Ordering::*;
        match Ord::cmp(&sum,&answer1) {
            Less => {
                let &e = iter.next().unwrap();
                ringbuf.push_front(e);
                sum += e;
            },
            Greater => {
                sum -= ringbuf.pop_back().unwrap();
            },
            Equal => {
                break
            }
        }
    }
    let min = ringbuf.iter().min().unwrap();
    let max = ringbuf.iter().max().unwrap();
    println!("Answer2: {}", min+max);
    Ok(())
}
