use std::collections::HashSet as Set;
use std::{collections::HashMap as Map, fmt::Write};

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut ingredient_names = Vec::new();
    let mut ingredient_idxs = Map::new();
    let mut allergen_names = Vec::new();
    let mut allergen_idxs = Map::new();
    let mut rules = Vec::new();
    for l in buf.lines() {
        let (ingrs, algns) = {
            let mut tmp = l.split('(');
            (
                tmp.next().unwrap(),
                tmp.next().unwrap().trim_end_matches(')'),
            )
        };
        let mut ingredients = Set::new();
        let mut allergens = Set::new();
        for i in ingrs.split_whitespace() {
            let idx = ingredient_idxs.entry(i).or_insert_with(|| {
                ingredient_names.push(i);
                ingredient_names.len() - 1
            });
            ingredients.insert(*idx);
        }
        for a in algns.split_whitespace().skip(1) {
            let a = a.trim_end_matches(',');
            let idx = allergen_idxs.entry(a).or_insert_with(|| {
                allergen_names.push(a);
                allergen_names.len() - 1
            });
            allergens.insert(*idx);
        }
        rules.push((ingredients, allergens));
    }
    // dbg!(&rules);
    let n_ingr = ingredient_names.len();
    let n_aller = allergen_names.len();
    let set_all_ingr: Set<_> = (0..n_ingr).collect();
    let mut possibly_contains = vec![Set::new(); n_ingr];
    for a in 0..n_aller {
        let possible_containers = rules
            .iter()
            .filter_map(|(rule_ingr, rule_all)| {
                if rule_all.contains(&a) {
                    Some(rule_ingr)
                } else {
                    None
                }
            })
            .fold(set_all_ingr.clone(), |acc, x| {
                acc.intersection(x).copied().collect()
            });
        // dbg!(allergen_names[a]);
        // dbg!(&possible_containers);
        for i in possible_containers {
            possibly_contains[i].insert(a);
        }
    }
    let mut a1: u64 = 0;
    for (i, possible_all) in possibly_contains.iter().enumerate() {
        // println!("\n{}",ingredient_names[i]);
        // for &a in possible_all {
        //     print!("{} ", allergen_names[a]);
        // }
        // dbg!(possible_all);
        if possible_all.is_empty() {
            for (rule_ingr, _) in &rules {
                if rule_ingr.contains(&i) {
                    // dbg!(ingredient_names[i]);
                    a1 += 1;
                }
            }
        }
    }
    println!("Answer1: {}", a1);
    let mut working_vec: Vec<_> = possibly_contains.iter().enumerate().collect();
    working_vec.sort_unstable_by_key(|&(_, a)| a.len());
    let mut free = vec![true; n_aller];
    let mut tmp = Vec::new();
    for (i, candidates) in working_vec {
        if candidates.is_empty() {
            continue;
        }
        let aux: Vec<_> = candidates.iter().filter(|&&a| free[a]).collect();
        let &contained = aux[0];
        free[contained] = false;
        tmp.push((ingredient_names[i], allergen_names[contained]));
    }
    tmp.sort_unstable_by_key(|&(_, a)| a);
    let mut a2 = String::new();
    for (i, _) in tmp {
        write!(a2, "{},", i)?;
    }
    println!("Answer 2:\n{}", a2.trim_end_matches(','));
    Ok(())
}
