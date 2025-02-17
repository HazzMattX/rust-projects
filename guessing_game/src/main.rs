use rand::Rng;
use std::io::Write;
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn play_game() {
    println!("Do you want to play a game?");
    let random_number: u8 = rand::thread_rng().gen_range(1..=100);
    let difficulty = get_input("Select difficulty: ");
    let difficulty = difficulty.trim();
    let mut attempts = match difficulty {
        "hard" => 5,
        "easy" => 10,
        _ => {
            println!("Wrong answer");
            0
        }
    };
    let mut guessed_correctly = false;
    loop {
        if attempts <= 0 {
            break;
        }
        println!("You have {attempts} attempts left.");
        let guess = get_input("Guess a number: ").trim().parse().unwrap_or(0);
        if guess > random_number {
            println!("Too high");
            attempts -= 1;
        } else if guess < random_number {
            println!("Too low");
            attempts -= 1;
        } else if guess == random_number {
            println!("Wow you got it right for once");
            guessed_correctly = true;
            break;
        }
    }
    if !guessed_correctly {
        println!("Wow you suck. The number was {random_number}.");
    }
    let play_again: String = get_input("Do you want to play again? ");
    let play_again = play_again.trim();
    match play_again {
        "yes" => play_game(),
        _ => println!("Bye bye"),
    }
}
fn main() {
    play_game();
}
