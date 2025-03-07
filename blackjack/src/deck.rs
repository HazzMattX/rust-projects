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
pub struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn deck() -> Deck {
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let mut cards = Deck { cards: Vec::new()};
        for suit in suits {
            for value in 2..11 {
                cards.cards.push(Card::new(value, suit.to_string()));
            }
            // Face cards
            cards.cards.push(Card::new(10, format!("Jack of {}", suit)));
            cards.cards.push(Card::new(10, format!("Queen of {}", suit)));
            cards.cards.push(Card::new(10, format!("King of {}", suit)));
            cards.cards.push(Card::new(11, format!("Ace of {}", suit))); // Ace starts at 11
        }
        let mut rng = rand::thread_rng();
        cards.cards.shuffle(&mut rng);
        Deck { cards: cards.cards }
    }
}
pub fn deal_cards(deck: &mut Deck, num_cards: usize) -> Vec<u8> {
    let mut hand = Vec::new();
    for _ in 0..num_cards {
        if let Some(card) = deck.cards.pop() {
            hand.push(card.value);
            println!("{}", deck.cards.len());
        }
    }
    hand
}