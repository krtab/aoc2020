use std::{error, fmt::Display, str::FromStr};

struct Pow2Iter {
    state: u64,
}

impl Pow2Iter {
    fn new() -> Pow2Iter {
        Pow2Iter { state: 1 }
    }
}

impl Iterator for Pow2Iter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == 0 {
            None
        } else {
            let res = self.state;
            self.state = self.state << 1;
            Some(res)
        }
    }
}

#[derive(Debug)]
enum SeatParseError {
    RowUnexpectedChar(char),
    ColUnexpectedChar(char),
}

impl Display for SeatParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SeatParseError::*;
        match self {
            RowUnexpectedChar(c) => {
                f.write_fmt(format_args!("Unexpected char {} when parsing row.",c))
            }
            ColUnexpectedChar(c) => {
                f.write_fmt(format_args!("Unexpected char {} when parsing column.",c))
            }
        }
    }
}
impl error::Error for SeatParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
struct Seat {
    row: u8,
    col: u8,
}

impl FromStr for Seat {
    type Err = SeatParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (row_str, col_str) = s.split_at(7);
        let mut row = 0;
        for (base, c) in Pow2Iter::new().zip(row_str.chars().rev()) {
            row += (base as u8)
                * match c {
                    'F' => 0,
                    'B' => 1,
                    _ => (return Err(SeatParseError::RowUnexpectedChar(c))),
                }
        }
        let mut col = 0;
        for (base, c) in Pow2Iter::new().zip(col_str.chars().rev()) {
            col += (base as u8)
                * match c {
                    'L' => 0,
                    'R' => 1,
                    _ => (return Err(SeatParseError::ColUnexpectedChar(c))),
                }
        }
        Ok(Seat { row, col })
    }
}

impl Seat {
    fn id(&self) -> u64 {
        (self.row as u64) * 8 + (self.col as u64)
    }
}
fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let seats: Result<Vec<Seat>, _> = buf.lines().map(|s| s.parse()).collect();
    let mut ids : Vec<_> = seats?.iter().map(Seat::id).collect();
    ids.sort_unstable();
    let maxid = ids.iter().max().unwrap();
    println!("Answer 1: {}", maxid);
    let  mut iter = ids.iter();
    let mut last = iter.next().unwrap();
    for id in iter {
        if id - last > 1 {
            println!("Answer2: {}",id-1);
            last = id
        } else {
            last = id;
        }
    }
    Ok(())
}
