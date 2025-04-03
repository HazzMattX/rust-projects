mod deck;
mod score;
mod funds;
use std::io::{self, Write};
use deck::deal_cards;
use score::*;
use funds::MONEY;
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
    println!("Cards remaining in deck: {}", deck.remaining());
    let mut dealer_score: u8;
    let mut user_score: u8;
    let mut playing = true;
    while playing {
        dealer_score = calculate_score(&dealer_cards);
        user_score = calculate_score(&user_cards);
        display_hand(&user_cards, user_score, true, true);
        display_hand(&dealer_cards, dealer_score, false, false);
        if user_score == 0 || dealer_score == 0 {
            playing = false;
        } else {
            if get_yes_no_input("Do you want to hit or stand? (y/n):") {
                user_cards.extend(deal_cards(&mut deck, 1));
                user_score = calculate_score(&user_cards);
                if user_score < 22 {
                    playing = true;
                } else if user_score > 21 {
                    playing = false;
                }
            } else {
                playing = false;
                while dealer_score != 0 && dealer_score < 17 {
                    dealer_cards.extend(deal_cards(&mut deck, 1));
                    dealer_score = calculate_score(&dealer_cards);
                }
            }
        }
        if !playing {
            println!("Cards remaining in deck: {}", deck.remaining());
            display_hand(&user_cards, user_score, true, true);
            display_hand(&dealer_cards, dealer_score, false, true);
            let result = compare(user_score, dealer_score);
            display_result(result);
        }
    }
    if get_yes_no_input("Do you want to play again? (y/n): ") {
        play_game();
    } else {
        println!("Thanks for playing!");
    }
}
// Example of more descriptive messages
fn display_result(result: GameResult) {
    if result.player_wins() {
        println!("Congratulations! {}", result.result());
    } else if result.dealer_wins() {
        println!("Better luck next time. {}", result.result());
    } else {
        println!("It's a draw!");
    }
}
fn display_hand(cards: &Vec<deck::Card>, score: u8, is_player: bool, show_all: bool) {
    let owner = if is_player { "Your" } else { "Dealer's" };
    if show_all {
        print!("{} cards: ", owner);
        for card in cards {
            print!("{} ", card);
        }
        let score_display = if score == 0 {
            "Blackjack!".to_string()
        } else {
            score.to_string()
        };
        println!(", score: {}", score_display);
    } else {
        // Only show the first card for dealer when show_all is false
        println!("{} first card: {}", owner, cards[0]);
    }
}
fn get_yes_no_input(prompt: &str) -> bool {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please type 'y' or 'n'."),
        }
    }
}
