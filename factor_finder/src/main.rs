use std::env;
fn main() {
    let mut ok_num: Vec<u64> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let num: u64 = args[1].parse().unwrap();
    let mut count = 0;
    for i in 1..=num {
        if num % i == 0 {
            count += 1;
            ok_num.push(i);
        }
    }
    if count == 2 {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
        println!("Perfect divisors include {:?}", ok_num);
    }
}
