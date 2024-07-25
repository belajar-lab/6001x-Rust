use std::collections::HashSet;

use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const WORDLIST_FILENAME: &str = "hangman/data/words.txt";

/// Loads words from a file.
///
/// This function reads words from a file named "words.txt".
/// Words are expected to be lowercase letters, separated by whitespace.
///
/// **Returns**:
/// - A vector of strings containing the loaded words.
///
/// **Panics**:
/// - Panics if the file cannot be opened, is empty, or if reading fails.
fn load_words() -> Vec<String> {
    println!("Loading word list from file...");

    let path = Path::new(WORDLIST_FILENAME);
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let wordlist: Vec<String> = reader
        .lines()
        .next()
        .expect("File is empty")
        .expect("Failed to read line")
        .split_whitespace()
        .map(String::from)
        .collect();

    println!("  {} words loaded.", wordlist.len());
    wordlist
}

/// Chooses a random word from the given word list.
///
/// **Arguments**:
/// * `wordlist` - A slice of strings to choose from.
///
/// **Returns**:
/// * A reference to a randomly selected word from the list.
///
/// **Panics**:
/// - Panics if the wordlist is empty.
pub fn random_word() -> String {
    let wordlist = load_words();
    wordlist
        .choose(&mut rand::thread_rng())
        .expect("Wordlist is empty")
        .to_string()
}

/// Checks if all the letters of the secret word have been guessed.
///
/// **Arguments:**
/// * `secret_word` - A string slice that holds the word the user is guessing.
/// * `letters_guessed` - A slice of characters that have been guessed so far.
///
/// **Returns:**
/// * `true` if all the letters of the secret word are in the letters guessed.
/// * `false` otherwise.
///
/// # Examples
///
/// ```
/// let secret_word = "apple";
/// let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];
///
/// let answer = hangman::is_word_guessed(secret_word, &letters_guessed);
/// assert_eq!(answer, false)
/// ```
pub fn is_word_guessed(secret_word: &str, letters_guessed: &[char]) -> bool {
    let secret_word_set: HashSet<char> = secret_word.chars().collect();
    let letters_guessed_set: HashSet<char> = letters_guessed.iter().cloned().collect();
    secret_word_set.is_subset(&letters_guessed_set)
}

/// Returns a string representation of the secret word with guessed letters revealed
/// and unguessed letters as underscores.
///
/// **Arguments:**
/// * `secret_word` - A string slice that holds the word the user is guessing.
/// * `letters_guessed` - A slice of characters that have been guessed so far.
///
/// **Returns:**
/// * A `String` comprised of letters and underscores that represents what letters
/// in the secret word have been guessed so far.
///
/// # Example:

/// ```
/// let secret_word = "apple";
/// let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];
///
/// let result = hangman::get_guessed_word(secret_word, &letters_guessed);
/// assert_eq!(result, "_pp_e".to_string())
/// ```
pub fn get_guessed_word(secret_word: &str, letters_guessed: &[char]) -> String {
    secret_word
        .chars()
        .map(|c| if letters_guessed.contains(&c) { c } else { '_' })
        .collect()
}

/// Returns a string of available letters that have not been guessed.
///
/// **Arguments**:
/// * `letters_guessed` - A slice of characters representing the letters that have been guessed so far.
///
/// **Returns**:
/// * A String containing all letters of the alphabet that have not been guessed, in alphabetical order.
///
/// # Example
/// ```
/// let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];
///
/// let answer = hangman::get_available_letters(&letters_guessed);
/// assert_eq!(answer, "abcdfghjlmnoqtuvwxyz");
/// ```
pub fn get_available_letters(letters_guessed: &[char]) -> String {
    ('a'..='z')
        .filter(|&c| !letters_guessed.contains(&c))
        .collect()
}
