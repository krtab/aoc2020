#![feature(iter_map_while)]

use std::collections::HashSet as Set;
use std::collections::VecDeque;

type Deck = VecDeque<u64>;

fn recursvie_combat(mut deck1: Deck, mut deck2: Deck) -> (bool, Deck) {
    let mut hist = Set::new();
    loop {
        // dbg!(&deck1,&deck2);
        if deck1.is_empty() {
            break (false, deck2);
        }
        if deck2.is_empty() {
            break (true, deck1);
        }
        if !hist.insert((deck1.clone(), deck2.clone())) {
            break (true, deck1);
        };
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();
        let p1_wins = {
            if c1 <= deck1.len() as u64 && c2 <= deck2.len() as u64 {
                let subdeck1: Deck = deck1.iter().take(c1 as usize).copied().collect();
                let subdeck2: Deck = deck2.iter().take(c2 as usize).copied().collect();
                let (p1_wins, _) = recursvie_combat(subdeck1, subdeck2);
                p1_wins
            } else {
                c1 >= c2
            }
        };
        if p1_wins {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }
}

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut iter = buf.lines();
    let input_deck1: Vec<_> = (&mut iter).skip(1).map_while(|s| s.parse().ok()).collect();
    let input_deck2: Vec<_> = iter.skip(1).map_while(|s| s.parse().ok()).collect();
    let mut working_deck1 = Deck::from(input_deck1.clone());
    let mut working_deck2 = Deck::from(input_deck2.clone());
    let winner_deck = loop {
        if working_deck1.is_empty() {
            break working_deck2;
        }
        if working_deck2.is_empty() {
            break working_deck1;
        }
        let c1 = working_deck1.pop_front().unwrap();
        let c2 = working_deck2.pop_front().unwrap();
        if c1 >= c2 {
            working_deck1.push_back(c1);
            working_deck1.push_back(c2);
        } else {
            working_deck2.push_back(c2);
            working_deck2.push_back(c1);
        }
    };
    let a1: u64 = winner_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| ((i as u64) + 1) * c)
        .sum();
    println!("Answer1: {}", a1);
    let (_, winner_deck) = recursvie_combat(input_deck1.into(), input_deck2.into());
    let a2: u64 = winner_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| ((i as u64) + 1) * c)
        .sum();
    println!("Answer2: {}", a2);
    Ok(())
}
