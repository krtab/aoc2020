#![feature(array_value_iter)]

use std::collections::HashSet as Set;
use std::{array::IntoIter, collections::HashMap as Map};

type T = i64;

const WHITE: bool = false;
const BLACK: bool = !WHITE;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct HexaTile {
    x: T,
    y: T,
}

impl HexaTile {
    fn adjacents(&self) -> [HexaTile; 6] {
        let x = self.x;
        let y = self.y;
        [
            HexaTile { x: x - 2, y: y },
            HexaTile { x: x + 2, y: y },
            HexaTile { x: x + 1, y: y + 1 },
            HexaTile { x: x + 1, y: y - 1 },
            HexaTile { x: x - 1, y: y + 1 },
            HexaTile { x: x - 1, y: y - 1 },
        ]
    }
}

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut state = Map::new();
    for l in buf.lines() {
        let mut x = 0;
        let mut y = 0;
        let mut chars = l.chars();
        while let Some(c) = chars.next() {
            // e, se, sw, w, nw, and ne
            match c {
                'e' => x += 2,
                'w' => x -= 2,
                's' => {
                    y -= 1;
                    match chars.next().unwrap() {
                        'e' => x += 1,
                        'w' => x -= 1,
                        _ => unreachable!(),
                    }
                }
                'n' => {
                    y += 1;
                    match chars.next().unwrap() {
                        'e' => x += 1,
                        'w' => x -= 1,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        let tile = HexaTile { x, y };
        *state.entry(tile).or_insert(WHITE) ^= true;
    }
    let a1 = state.values().filter(|&&x| x == BLACK).count();
    println!("Answer1: {}", a1);
    for i in 1..=100 {
        let mut to_check: Set<HexaTile> = state
            .keys()
            .flat_map(|t| IntoIter::new(t.adjacents()))
            .collect();
        to_check.extend(state.keys());
        let mut changes = Vec::new();
        for tile in to_check {
            let adj_blacks = tile
                .adjacents()
                .iter()
                .filter_map(|t| state.get(t))
                .filter(|&&color| color == BLACK)
                .count();
            let &self_col = state.get(&tile).unwrap_or(&WHITE);
            if self_col == BLACK && (adj_blacks == 0 || adj_blacks > 2) {
                changes.push((tile, WHITE));
            }
            if self_col == WHITE && adj_blacks == 2 {
                changes.push((tile, BLACK));
            }
        }
        for (tile, color) in changes {
            state.insert(tile, color);
        }
        println!(
            "Day {}: {}/{}",
            i,
            state.values().filter(|&&x| x == BLACK).count(),
            state.len()
        );
    }
    Ok(())
}
