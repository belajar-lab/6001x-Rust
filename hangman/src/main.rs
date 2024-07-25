use hangman::{get_available_letters, get_guessed_word, is_word_guessed, random_word};
use std::io::{self, Write};

/// Starts an interactive game of Hangman.
///
/// # Description
///
/// This function implements the game of Hangman with the following rules:
/// * The user is informed of the length of the secret word at the start.
/// * The user is asked to guess one letter per round.
/// * Feedback is provided immediately after each guess.
/// * The partially guessed word and available letters are displayed after each round.
/// * The game ends when the word is guessed or when the user runs out of guesses.
fn main() {
    let secret_word = random_word();

    println!("Welcome to the game Hangman!");
    println!(
        "I am thinking of a word that is {} letters long.",
        secret_word.len()
    );

    let mut letters_guessed = Vec::new();
    let mut guesses = 8;

    while !is_word_guessed(&secret_word, &letters_guessed) && guesses > 0 {
        println!("-----------");
        println!("You have {} guesses left.", guesses);
        println!(
            "Available Letters: {}",
            get_available_letters(&letters_guessed)
        );

        print!("Please guess a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap_or('\0');

        if letters_guessed.contains(&guess) {
            println!(
                "Oops! You've already guessed that letter: {}",
                get_guessed_word(&secret_word, &letters_guessed)
            );
        } else if get_available_letters(&letters_guessed).contains(guess) {
            letters_guessed.push(guess);
            if secret_word.contains(guess) {
                println!(
                    "Good guess: {}",
                    get_guessed_word(&secret_word, &letters_guessed)
                );
            } else {
                println!(
                    "Oops! That letter is not in my word: {}",
                    get_guessed_word(&secret_word, &letters_guessed)
                );
                guesses -= 1;
            }
        }
    }

    println!("-----------");
    if guesses > 0 {
        println!("Congratulations, you won!");
    } else {
        println!(
            "Sorry, you ran out of guesses. The word was {}.",
            secret_word
        );
    }
}
