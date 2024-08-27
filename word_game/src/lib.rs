const VOWELS: &str = "aeiou";
const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyz";
const HAND_SIZE: usize = 7;

const SCRABBLE_LETTER_VALUES: [i32; 26] = [
    1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1,
    3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10
];

fn get_word_score(word: &str, n: usize) -> i32 {
    if c.is_ascii_lowercase() {
        SCRABBLE_LETTER_VALUES[(c as u8 - b'a') as usize]
    } else {
        0
    }
}