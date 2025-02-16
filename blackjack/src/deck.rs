// Creates a deck of cards for a blackjack game.
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Card {
    value: u8,
    suit: String,
}
impl Card {
    fn new(value: u8, suit: String) -> Card {
        Card { value, suit }
    }
}
fn new_deck() -> Vec<Card> {
    let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
    let mut deck = Vec::new();
    for suit in suits {
        for value in 2..11 {
            deck.push(Card::new(value, suit.to_string()));
        }
        // Face cards
        deck.push(Card::new(10, format!("Jack of {}", suit)));
        deck.push(Card::new(10, format!("Queen of {}", suit)));
        deck.push(Card::new(10, format!("King of {}", suit)));
        deck.push(Card::new(11, format!("Ace of {}", suit))); // Ace starts at 11
    }
    deck
}
pub fn deal_cards(num_cards: usize) -> Vec<u8> {
    let mut deck = new_deck();
    let mut rng = rand::thread_rng();
    let mut hand = Vec::new();
    for _ in 0..num_cards {
        if let Some(card) = deck.choose_mut(&mut rng) {
            hand.push(card.value);
        }
    }
    hand
}
