use num_bigint::BigInt;

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut lines = buf.lines();
    let earliest : u64 = lines.next().unwrap().parse()?;
    let ids : Vec<_> = lines.next().unwrap().split(',').collect();
    let mut min = u64::MAX;
    let mut argmin = u64::MAX;
    for id in &ids {
        match id.parse() {
            Err(_) => continue,
            Ok(n) => {
                let tmp = n - earliest%n;
                if tmp < min {
                    min = tmp;
                    argmin = n;
                }
            }
        }
    }
    println!("Answer1: {}", min*argmin);
    let mut us = Vec::new();
    let mut ms = Vec::new();
    for (i,id) in ids.iter().enumerate() {
        match id.parse::<u64>() {
            Err(_) => continue,
            Ok(n) => {
                us.push(BigInt::from(-(i as i64)));
                ms.push(BigInt::from(n));
            }
        }
    };
    let foo = ring_algorithm::chinese_remainder_theorem(&us, &ms).unwrap();
    let lcm = ms.into_iter().fold(BigInt::from(1), num::integer::lcm);
    println!("Answer2: {:?}", foo+lcm);
    Ok(())
}
