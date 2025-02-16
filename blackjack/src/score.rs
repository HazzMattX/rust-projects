// This page calculates and compares the scores of a blackjack game.
pub fn calculate_score(cards: &mut Vec<u8>) -> u8 {
    if cards.iter().sum::<u8>() == 21 && cards.len() == 2 {
        // Blackjack! You win!
        return 0;
    }
    if cards.contains(&11) && cards.iter().sum::<u8>() > 21 {
        // Convert the first ace to a 1 if the sum is over 21
        if let Some(pos) = cards.iter().position(|&x| x == 11) {
            cards[pos] = 1;
        }
    }
    cards.iter().sum::<u8>()
}
pub fn compare(your_score: u8, house_score: u8) -> &'static str {
    match (your_score, house_score) {
        (0, _) => "Blackjack! You win!",
        (_, 0) => "Dealer has Blackjack! You lose!",
        (ys, hs) if ys > 21 => "You bust! You lose!",
        (ys, hs) if hs > 21 => "Dealer bust! You win!",
        (ys, hs) if ys > hs => "You win!",
        (ys, hs) if ys < hs => "Dealer wins!",
        _ => "Draw",
    }
}
