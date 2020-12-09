use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let buf = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut group_n_reply_or = Vec::new();
    let mut group_n_reply_and = Vec::new();
    for group in buf.split("\n\n") {
        let mut answers = HashMap::new();
        let mut persons_count = 0_u64;
        for person in group.split_whitespace() {
            persons_count+=1;
            for c in person.chars() {
                let counter = answers.entry(c).or_insert(0_u64);
                *counter += 1;
            }
        }
        group_n_reply_or.push(answers.len());
        let reply_and = answers.values().filter(|&x| *x == persons_count).count();
        group_n_reply_and.push(reply_and);
        
    }
    let a1 : usize = group_n_reply_or.iter().sum();
    println!("Answer1: {}", a1 );
    let a2 : usize = group_n_reply_and.iter().sum();
    println!("Answer1: {}", a2 );
    Ok(())
}
