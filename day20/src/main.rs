use regex::Regex;
use std::{
    collections::HashMap,
    convert::{identity, TryInto},
    mem::{size_of, size_of_val},
};
use std::{fmt::Display, str::FromStr};

type TileId = u8;
const TILE_SIZE: usize = 10;
const IMAGE_SIZE_IN_TILES: usize = 12;

// 3x20
const MONSTER: [u128; 3] = [
    0b00000000000000000010 << (128 - 20),
    0b10000110000110000111 << (128 - 20),
    0b01001001001001001000 << (128 - 20),
];

#[derive(Debug, Clone, Copy)]
enum BorderParseError {
    UnknownChar(char),
}

impl Display for BorderParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorderParseError::UnknownChar(c) => f.write_fmt(format_args!("Unknown char {}.", c)),
        }
    }
}

impl std::error::Error for BorderParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Border(u16);

impl FromStr for Border {
    type Err = BorderParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = 0;
        for c in s.chars() {
            let tmp = match c {
                '.' => 0,
                '#' => 1,
                c => return Err(BorderParseError::UnknownChar(c)),
            };
            res = (res << 1) + tmp;
        }
        Ok(Border(res))
    }
}

impl Border {
    fn rev(self) -> Border {
        Border(self.0.reverse_bits() >> (size_of_val(&self.0) * 8 - TILE_SIZE))
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Top = 0,
    Left = 1,
    Bottom = 2,
    Right = 3,
}

impl Position {
    fn idx(&self) -> usize {
        *self as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    D0 = 0,
    D90 = 1,
    D180 = 2,
    D270 = 3,
}

impl From<u8> for Rotation {
    fn from(x: u8) -> Self {
        match x % 4 {
            0 => Rotation::D0,
            1 => Rotation::D90,
            2 => Rotation::D180,
            3 => Rotation::D270,
            _ => unreachable!(),
        }
    }
}
impl std::ops::Add for Rotation {
    type Output = Rotation;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from((self as u8) + (rhs as u8))
    }
}

impl Rotation {
    fn quarters(&self) -> usize {
        *self as usize
    }

    fn next(self) -> Rotation {
        match self {
            Rotation::D0 => Rotation::D90,
            Rotation::D90 => Rotation::D180,
            Rotation::D180 => Rotation::D270,
            Rotation::D270 => Rotation::D0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Tile {
    id: TileId,
    rotation: Rotation,
    h_flipped: bool,
    borders: [Border; 4],
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let flip_n = self.h_flipped as u8;
        f.write_fmt(format_args!(
            "{:3}-{}-{}",
            self.id,
            self.rotation.quarters(),
            flip_n
        ))
    }
}

impl Tile {
    fn border(&self, pos: Position) -> Border {
        self.borders[pos.idx()]
    }

    fn rotate(self, rot: Rotation) -> Tile {
        Tile {
            rotation: self.rotation + rot,
            borders: {
                let mut tmp = self.borders;
                tmp.as_mut().rotate_right(rot as usize);
                tmp
            },
            ..self
        }
    }

    fn h_flip(self) -> Tile {
        use Position::*;
        Tile {
            h_flipped: !self.h_flipped,
            borders: [
                self.border(Top).rev(),
                self.border(Right).rev(),
                self.border(Bottom).rev(),
                self.border(Left).rev(),
            ],
            ..self
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Interior([u8; 8]);

impl Interior {
    // (i,j) -> (7-j,i) en coordonnée images
    // (i,7-j) -> (7-j,7-i) en coordonnées "bits"
    // ie (i,j) -> (j,7-i)
    fn rotate90(&self) -> Self {
        let mut res = [0; 8];
        for (i, orig_row) in self.0.iter().enumerate() {
            for (j, dest_row) in res.iter_mut().enumerate() {
                *dest_row |= (((1 << j) & orig_row) >> j) << (7 - i)
            }
        }
        Interior(res)
    }

    fn h_flip(mut self) -> Self {
        for row in self.0.iter_mut() {
            *row = row.reverse_bits()
        }
        self
    }
}

fn main() -> anyhow::Result<()> {
    let re_tile_id = Regex::new(r"^Tile (\d+):$")?;
    let mut names = HashMap::new();
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut iter = buf.lines();
    let mut tiles = Vec::new();
    let mut interiors = Vec::new();
    while let Some(l) = iter.next() {
        let tile_name: u16 = re_tile_id
            .captures(l)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()?;
        let tile_id = names.len() as TileId;
        names.insert(tile_id, tile_name);
        let mut rev_left = String::new();
        let mut right = String::new();
        let top = iter.next().unwrap();
        rev_left.push(top.chars().next().unwrap());
        right.push(top.chars().last().unwrap());
        let mut inter = Vec::new();
        for _ in 1..(TILE_SIZE - 1) {
            let l_prime = iter.next().unwrap();
            rev_left.push(l_prime.chars().next().unwrap());
            right.push(l_prime.chars().last().unwrap());
            inter.push(
                l_prime
                    .chars()
                    .skip(1)
                    .take(8)
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!(),
                    })
                    .fold(0_u8, |acc, x| (acc * 2) + x),
            )
        }
        interiors.push(Interior(inter.try_into().unwrap()));
        let rev_bot = iter.next().unwrap();
        rev_left.push(rev_bot.chars().next().unwrap());
        right.push(rev_bot.chars().last().unwrap());
        // newline
        iter.next();
        let bot: String = rev_bot.chars().rev().collect();
        let left: String = rev_left.chars().rev().collect();
        tiles.push(Tile {
            id: tile_id,
            h_flipped: false,
            rotation: Rotation::D0,
            borders: [top.parse()?, left.parse()?, bot.parse()?, right.parse()?],
        })
    }
    let mut has_reversed_topleft = HashMap::new();
    let mut has_reversed_left = HashMap::new();
    let mut has_reversed_top = HashMap::new();
    let mut extended_tiles = Vec::new();
    for &tile in &tiles {
        // dbg!(tile);
        for hfl in &[Tile::h_flip, identity] {
            for &rot in &[Rotation::D0, Rotation::D90, Rotation::D180, Rotation::D270] {
                let t = hfl(tile).rotate(rot);
                // dbg!(t);
                let rev_top = t.border(Position::Top).rev();
                debug_assert!(rev_top.0 >> TILE_SIZE == 0);
                let rev_left = t.border(Position::Left).rev();
                debug_assert!(rev_left.0 >> TILE_SIZE == 0);
                for entr in [
                    has_reversed_topleft
                        .entry((rev_top, rev_left))
                        .or_insert_with(Vec::new),
                    has_reversed_left.entry(rev_left).or_insert_with(Vec::new),
                    has_reversed_top.entry(rev_top).or_insert_with(Vec::new),
                ]
                .iter_mut()
                {
                    entr.push(t)
                }
                extended_tiles.push(t);
            }
        }
    }
    let mut stack: Vec<(_, Vec<Tile>)> = vec![(vec![true; names.len()], Vec::new())];
    let mut arrangements = Vec::new();
    while let Some((availables, state)) = stack.pop() {
        let already_in_place = state.len();
        // println!("{} {}",already_in_place,availables.iter().filter(|&&x| x).count());
        if already_in_place == availables.len() {
            arrangements.push(state);
            continue;
        }
        let candidates = match (
            already_in_place < IMAGE_SIZE_IN_TILES,
            (already_in_place % IMAGE_SIZE_IN_TILES) == 0,
        ) {
            (true, true) => {
                // println!("TopLeft");
                Some(&extended_tiles)
            }
            (true, false) => {
                // println!("FirstLine");
                has_reversed_left.get(&state.last().unwrap().border(Position::Right))
            }
            (false, true) => {
                // println!("FirstCol");
                has_reversed_top
                    .get(&state[already_in_place - IMAGE_SIZE_IN_TILES].border(Position::Bottom))
            }
            (false, false) => has_reversed_topleft.get(&(
                state[already_in_place - IMAGE_SIZE_IN_TILES].border(Position::Bottom),
                state.last().unwrap().border(Position::Right),
            )),
        };
        // println!("From state: {:?}", state);
        if candidates.is_none() {
            // println!("No candidate");
        }
        for try_tile in candidates.into_iter().flatten() {
            // print!("Trying {}: ", try_tile);
            if !availables[try_tile.id as usize] {
                // println!("Not available.");
                continue;
            }
            let mut new_availables = availables.clone();
            new_availables[try_tile.id as usize] = false;
            let mut new_state = state.clone();
            new_state.push(try_tile.clone());
            // println!("Available, pushed.");
            stack.push((new_availables, new_state));
        }
    }
    let arrang1 = &arrangements[0];
    let a1: u64 = [
        0,
        IMAGE_SIZE_IN_TILES - 1,
        IMAGE_SIZE_IN_TILES.pow(2) - IMAGE_SIZE_IN_TILES,
        IMAGE_SIZE_IN_TILES.pow(2) - 1,
    ]
    .iter()
    .map(|&i| names[&arrang1[i].id] as u64)
    .product();
    println!("Answer1: {}", a1);
    for arr in &arrangements {
        let mut img = [0_u128; 8 * IMAGE_SIZE_IN_TILES];
        for (i, tile) in arr.iter().enumerate() {
            let mut inter = interiors[tile.id as usize];
            if tile.h_flipped {
                inter = inter.h_flip();
            }
            for _ in 0..(tile.rotation as u8) {
                inter = inter.rotate90();
            }
            for (j, &row) in inter.0.iter().enumerate() {
                img[(i / IMAGE_SIZE_IN_TILES) * 8 + j] |=
                    (row as u128) << ((128 - 8) - 8 * (i % IMAGE_SIZE_IN_TILES));
            }
        }
        let mut count_monster: u64 = 0;
        for (i, three_rows) in img.windows(3).enumerate() {
            let pattern = MONSTER;
            for offset in 0..(8 * IMAGE_SIZE_IN_TILES - 20) {
                let omg_a_monster = three_rows.iter().zip(&pattern).all(|(&r, pat)| {
                    let p = pat >> offset;
                    !(!p | r) == 0
                });
                if omg_a_monster {
                    count_monster += 1;
                }
            }
        }
        if count_monster > 0 {
            let mut a2: u64 = 0;
            for row in &img {
                a2 += row.count_ones() as u64;
            }
            let tmp : u64 = MONSTER.iter().map(|r| r.count_ones() as u64).sum();
            a2 -= count_monster * tmp;
            println!("Answer2: {}", a2);
            break
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Border, Interior, Rotation, Tile};

    #[test]
    fn rev_idempotent() {
        let bor = Border(0b000000_1001100010);
        assert_eq!(bor.rev().rev(), bor)
    }

    #[test]
    fn rot1() {
        let tile = Tile {
            id: 0,
            rotation: Rotation::D0,
            h_flipped: false,
            borders: [Border(0), Border(8), Border(12), Border(15)],
        };
        assert_eq!(tile.rotate(Rotation::D0), tile);
        assert_eq!(
            tile.rotate(Rotation::D90).rotate(Rotation::D90),
            tile.rotate(Rotation::D180)
        );
        assert_eq!(
            tile.rotate(Rotation::D90)
                .rotate(Rotation::D90)
                .rotate(Rotation::D90)
                .rotate(Rotation::D90),
            tile
        );
    }

    #[test]
    fn flip1() {
        let tile = Tile {
            id: 0,
            rotation: Rotation::D0,
            h_flipped: false,
            borders: [Border(0), Border(8), Border(12), Border(15)],
        };
        assert_eq!(tile.h_flip().h_flip(), tile);
    }

    #[test]
    fn roate_interior1() {
        let orig = Interior([
            0b1000_0000,
            0b1000_0000,
            0b0000_0001,
            0b0011_0000,
            //
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0001_1000,
        ]);
        let res = Interior([
            0b0010_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
            //
            0b0001_0001,
            0b0001_0000,
            0b0000_0000,
            0b1100_0000,
        ]);
        assert_eq!(orig.rotate90(), res);
    }

    #[test]
    fn flip_interior() {
        let orig = Interior([
            0b1000_0000,
            0b1000_0000,
            0b0000_0001,
            0b0011_0000,
            //
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ]);
        let res = Interior([
            0b0000_0001,
            0b0000_0001,
            0b1000_0000,
            0b0000_1100,
            //
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ]);
        assert_eq!(orig.h_flip(), res);
    }
}
