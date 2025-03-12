use rand::seq::SliceRandom;
use rayon::prelude::*;
const SUITS: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];
#[derive(Debug, PartialEq)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
}
impl Card {
    fn new(rank: Rank) -> Card {
        Card { rank }
    }
}
pub struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn new_deck() -> Deck {
                    let mut cards: Vec<Card> = SUITS
                        .par_iter()
                        .flat_map(|_suit| {
                            vec![
                                Card::new(Rank::Two),
                                Card::new(Rank::Three),
                                Card::new(Rank::Four),
                                Card::new(Rank::Five),
                                Card::new(Rank::Six),
                                Card::new(Rank::Seven),
                                Card::new(Rank::Eight),
                                Card::new(Rank::Nine),
                                Card::new(Rank::Ten),
                                Card::new(Rank::Jack),
                                Card::new(Rank::Queen),
                                Card::new(Rank::King),
                                Card::new(Rank::Ace),
                            ]
                        })
                        .collect();
                    let mut rng = rand::thread_rng();
                    cards.shuffle(&mut rng);
                    Deck { cards }
                }
}
pub fn deal_cards(deck: &mut Deck, num_cards: usize) -> Vec<Card> {
    let mut hand = Vec::new();
    for _ in 0..num_cards {
        if let Some(card) = deck.cards.pop() {
            hand.push(card);
            println!("{}", deck.cards.len());
        }
    }
    hand
}
