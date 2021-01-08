use std::iter::{successors, Successors};

type T = u64;

fn step(x: T, subject_number: T) -> T {
    (x * subject_number) % 20201227
}

fn crypto_iter(subject_number: T) -> Successors<T, impl FnMut(&T) -> Option<T>> {
    successors(Some(1), move |&x| Some(step(x, subject_number)))
}

fn main() -> anyhow::Result<()> {
    let (key1,key2) = 
    // example
    // (5764801,17807724)
    // input
    (18356117,5909654)
    ;
    let (loop_size_one, _) = crypto_iter(7)
        .enumerate()
        .find(|&(_, x)| x == key1)
        .unwrap();
    let secret_key = crypto_iter(key2).nth(loop_size_one).unwrap();
    println!("Answer 1: {}", secret_key);
    Ok(())
}
