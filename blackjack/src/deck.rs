use rand::seq::SliceRandom;
use rayon::prelude::*;
use std::{fmt, fmt::Display};
// In deck.rs
#[derive(Debug, Clone, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
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
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
            Rank::Ace => write!(f, "A"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
pub struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    pub fn new_deck() -> Deck {
        let suits = vec![Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = vec![
            Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight,
            Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
        ];
        let mut cards: Vec<Card> = suits
                    .par_iter()
                    .flat_map(|suit| {
                        ranks.iter().map(|rank| Card::new(rank.clone(), suit.clone())).collect::<Vec<Card>>()
                    })
                    .collect();

                let mut rng = rand::thread_rng();
                cards.shuffle(&mut rng);

                Deck { cards }
    }
    pub fn remaining(&self) -> usize {
            self.cards.len()
    }
}

pub fn deal_cards(deck: &mut Deck, num_cards: usize) -> Vec<Card> {
    let mut hand = Vec::new();
    for _ in 0..num_cards {
        if let Some(card) = deck.cards.pop() {
            hand.push(card);
        } else {
            println!("Deck is empty! Reshuffling...");
            *deck = Deck::new_deck();
            if let Some(card) = deck.cards.pop() {
                hand.push(card);
            }
        }
    }
    hand
}
