use crate::deck::{Card, Rank};
#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    PlayerBlackjack,
    DealerBlackjack,
    PlayerBust,
    DealerBust,
    PlayerWin,
    DealerWin,
    Draw,
}
impl GameResult {
    pub fn to_string(&self) -> &'static str {
        match self {
            GameResult::PlayerBlackjack => "Player Blackjack",
            GameResult::DealerBlackjack => "Dealer Blackjack",
            GameResult::PlayerBust => "Player Bust",
            GameResult::DealerBust => "Dealer Bust",
            GameResult::PlayerWin => "Player Win",
            GameResult::DealerWin => "Dealer Win",
            GameResult::Draw => "Draw",
        }
    }
    pub fn player_wins(&self) -> bool {
        matches!(self, GameResult::PlayerBlackjack | GameResult::PlayerWin | GameResult::DealerBust)
    }
    pub fn dealer_wins(&self) -> bool {
        matches!(self, GameResult::DealerBlackjack | GameResult::DealerWin | GameResult::PlayerBust)
    }
}
pub fn calculate_score(cards: &Vec<Card>) -> u8 {
    let mut score = 0;
    let mut aces = 0;
    for card in cards {
        match card.rank {
            Rank::Two => score += 2,
            Rank::Three => score += 3,
            Rank::Four => score += 4,
            Rank::Five => score += 5,
            Rank::Six => score += 6,
            Rank::Seven => score += 7,
            Rank::Eight => score += 8,
            Rank::Nine => score += 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => score += 10,
            Rank::Ace => {
                score += 11;
                aces += 1;
            }
        }
    }
    while score > 21 && aces > 0 {
        score -= 10;
        aces -= 1;
    }
    if score == 21 && cards.len() == 2 {
        return 0; // Blackjack
    }
    score
}
pub fn compare(your_score: u8, house_score: u8) -> GameResult {
    use GameResult::*;
    match (your_score, house_score) {
        (0, _) => PlayerBlackjack,
        (_, 0) => DealerBlackjack,
        (ys, _) if ys > 21 => PlayerBust,
        (_, hs) if hs > 21 => DealerBust,
        (ys, hs) if ys > hs => PlayerWin,
        (ys, hs) if ys < hs => DealerWin,
        _ => Draw,
    }
}
