use regex::Regex;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let re_mask = Regex::new(r"^mask = ([X10]{36})$")?;
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$")?;
    {
        let mut mask: u64 = 0;
        let mut mask_vals: u64 = 0;
        let mut memory = HashMap::new();
        for l in buf.lines() {
            if let Some(cap) = re_mask.captures(l) {
                mask = 0;
                mask_vals = 0;
                for c in cap[1].chars() {
                    mask <<= 1;
                    mask_vals <<= 1;
                    match c {
                        'X' => {}
                        '1' => {
                            mask += 1;
                            mask_vals += 1
                        }
                        '0' => mask += 1,
                        _ => unimplemented!(),
                    }
                }
            } else if let Some(cap) = re_mem.captures(l) {
                let idx: u64 = cap[1].parse()?;
                let val: u64 = cap[2].parse()?;
                let masked = (mask & mask_vals) | (!mask & val);
                memory.insert(idx, masked);
            } else {
                panic!("Unknown instr");
            }
        }
        let a1: u64 = memory.values().sum();
        println!("Answer1: {}", a1);
    }
    let mut masks = Vec::new();
    let mut masks_floatting = 0_u64;
    let mut memory = HashMap::new();
    for l in buf.lines() {
        if let Some(cap) = re_mask.captures(l) {
            masks = vec![0_u64];
            masks_floatting = 0;
            for c in cap[1].chars() {
                masks_floatting <<= 1;
                masks_floatting += match c {
                    'X' => {
                        let mut new_masks = Vec::new();
                        for mut m in masks {
                            m <<= 1;
                            new_masks.push(m);
                            new_masks.push(m + 1);
                        }
                        masks = new_masks;
                        1
                    }
                    '1' | '0' => {
                        let x = if c == '1' { 1 } else { 0 };
                        for m in &mut masks {
                            *m <<= 1;
                            *m += x;
                        }
                        0
                    }
                    _ => unimplemented!(),
                }
            }
        } else if let Some(cap) = re_mem.captures(l) {
            let idx: u64 = cap[1].parse()?;
            let val: u64 = cap[2].parse()?;
            for m in &masks {
                let masked = (masks_floatting & m) | (!masks_floatting & (m | idx));
                memory.insert(masked, val);
            }
        } else {
            panic!("Unknown instr");
        }
    }
    let a2: u64 = memory.values().sum();
    println!("Answer1: {}", a2);
    Ok(())
}
