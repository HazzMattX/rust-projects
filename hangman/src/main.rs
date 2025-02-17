use rand::seq::SliceRandom;
mod hangman_art;
mod hangman_words;
fn main() {
    use hangman_art::*;
    println!("{}", LOGO);
    let mut lives = 6;
    let mut rng = rand::thread_rng();
    let word = hangman_words::WORD_LIST
        .choose(&mut rng)
        .expect("Must be from WORD_LIST");
    let word_length: u8 = word.len() as u8;
    let mut placeholder: Vec<char> = vec!['_'; word_length as usize];
    println!("Word to guess: {}", placeholder.iter().collect::<String>());
    let mut game_over = false;
    let mut correct_letters = Vec::new();
    while !game_over {
        println!("****************************{lives}/6 LIVES LEFT****************************");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Must be a letter");
        let guess = guess.trim().chars().next().unwrap();
        if correct_letters.contains(&guess) {
            println!("You have already guessed {guess}.");
        } else {
            let mut correct_guess = false;
            for (i, letter) in word.chars().enumerate() {
                if letter == guess {
                    placeholder[i] = letter;
                    correct_guess = true;
                }
            }
            if correct_guess {
                correct_letters.push(guess);
            } else {
                // Handle incorrect guess
                lives -= 1;
                if lives == 0 {
                    game_over = true;
                    println!(
                        "***********************YOU LOSE********************** \nWord was {word}"
                    );
                }
            }
        }
        println!("Word to guess: {}", placeholder.iter().collect::<String>());
        if !placeholder.contains(&'_') {
            game_over = true;
            println!("****************************YOU WIN****************************");
        }
        println!("{}", STAGES[lives as usize]);
    }
}
