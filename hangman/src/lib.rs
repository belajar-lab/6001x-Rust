use std::collections::HashSet;

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