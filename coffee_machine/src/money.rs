use std::sync::Mutex;use once_cell::sync::Lazy;
pub static MONEY: Lazy<Mutex<f64>> = Lazy::new(|| Mutex::new(0.0));
pub fn profit() {
    println!("Money: ${:.2}", MONEY.lock().unwrap());
}
pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
pub fn pay() -> f64 {
    let quarters = get_input("How many quarters?").trim().parse().unwrap_or(0.0);
    let dimes = get_input("How many dimes?").trim().parse().unwrap_or(0.0);
    let nickels = get_input("How many nickels?").trim().parse().unwrap_or(0.0);
    let pennies = get_input("How many pennies?").trim().parse().unwrap_or(0.0);
    let total = quarters * 0.25 + dimes * 0.10 + nickels * 0.05 + pennies * 0.01;
    total
}
pub fn is_enough_money(mut total: f64, cost: f64) -> bool {
    let is_enough = total >= cost;
    if is_enough {
        *MONEY.lock().unwrap() += cost;
        total -= cost;
        println!("Here's your change: ${:.2}", total);
    }
    is_enough
}
