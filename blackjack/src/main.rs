mod deck;
mod score;
use deck::deal_cards;
use score::*;
fn main() {
    play_game();
}
fn play_game() {
    // Initialize game variables
    let mut deck = deck::Deck::new_deck();
    let mut user_cards = Vec::new();
    let mut dealer_cards = Vec::new();
    user_cards.extend(deal_cards(&mut deck, 2));
    dealer_cards.extend(deal_cards(&mut deck, 2));
    let mut dealer_score: u8;
    let mut user_score: u8;
    let mut playing = true;
    while playing {
        dealer_score = calculate_score(&mut dealer_cards);
        user_score = calculate_score(&mut user_cards);
        println!(
            "Your cards: {:?}, current score: {:?}",
            user_cards, user_score
        );
        println!("Dealer's first card: {:?}", dealer_cards[0]);
        if user_score == 0 || dealer_score == 0 {
            playing = false;
        } else {
            let hit_or_stand = get_input("Hit or stand? Type 'y' to hit or 'n' to stand");
            if hit_or_stand == "y" {
                user_cards.extend(deal_cards(&mut deck, 1));
                user_score = calculate_score(&mut user_cards);
                if user_score < 22 {
                    playing = true;
                } else if user_score > 21 {
                    playing = false;
                }
            } else if hit_or_stand == "n" {
                playing = false;
            }
            if hit_or_stand == "n" {
                while dealer_score != 0 && dealer_score < 17 {
                    dealer_cards.extend(deal_cards(&mut deck, 1));
                    dealer_score = calculate_score(&mut dealer_cards);
                }
            }
        }
        if !playing {
            println!("Your cards: {:?}, Your score: {:?}", user_cards, user_score);
            println!(
                "Dealer's cards: {:?}, Dealer's score: {:?}",
                dealer_cards, dealer_score
            );
            println!("{:?}", compare(user_score, dealer_score));
        }
    }
    let play_again = get_input("Game over! Do you want to play again? Type 'y' to play again or 'n' to exit");
    if play_again == "y" {
        play_game();
    } else {
        println!("Thanks for playing!");
    }
}
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
