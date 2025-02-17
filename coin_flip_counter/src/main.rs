use rand::seq::SliceRandom;
use std::collections::HashMap;
fn main() {
    let flip_outcomes = vec!["heads", "tails"];
    let mut total_flips = HashMap::new();
    let mut rng = rand::thread_rng();
    println!("Choose a number");
    let mut times_flipped = String::new();
    std::io::stdin()
        .read_line(&mut times_flipped)
        .expect("Must be a number");
    let times_flipped: u64 = times_flipped
        .trim()
        .parse()
        .expect("Failed to parse number");
    for _ in 1..=times_flipped {
        let flip = flip_outcomes
            .choose(&mut rng)
            .expect("must choose an outcome");
        let value = total_flips.entry(flip).or_insert(0);
        *value += 1;
    }
    for (flip, number) in total_flips {
        println!("{flip}: {number}");
    }
}
