use hangman::is_word_guessed;

#[test]
fn test_1() {
    let secret_word = "apple";
    let letters_guessed = vec!['a', 'e', 'i', 'k', 'p', 'r', 's'];

    let result = is_word_guessed(secret_word, &letters_guessed);
    assert_eq!(result, false)
}

#[test]
fn test_2() {
    let secret_word = "durian";
    let letters_guessed = vec!['h', 'a', 'c', 'd', 'i', 'm', 'n', 'r', 't', 'u'];

    let result = is_word_guessed(secret_word, &letters_guessed);
    assert_eq!(result, true)
}

#[test]
fn random_test_1() {
    let secret_word = "pineapple";
    let letters_guessed = vec!['v', 'l', 'c', 'o', 'h', 'x', 'n', 'b', 'y', 'd'];

    let result = is_word_guessed(secret_word, &letters_guessed);
    assert_eq!(result, false)
}

#[test]
fn random_test_2() {
    let secret_word = "broccoli";
    let letters_guessed = vec!['f', 'v', 'h', 'c', 'p', 'w', 'a', 'z', 'l', 'o'];

    let result = is_word_guessed(secret_word, &letters_guessed);
    assert_eq!(result, false)
}

#[test]
fn random_test_3() {
    let secret_word = "grapefruit";
    let letters_guessed = vec![];

    let result = is_word_guessed(secret_word, &letters_guessed);
    assert_eq!(result, false)
}

#[test]
fn random_test_4() {
    let secret_word = "coconut";
    let letters_guessed = vec!['z', 'x', 'q', 'c', 'o', 'c', 'o', 'n', 'u', 't'];

    let result = is_word_guessed(secret_word, &letters_guessed);
    assert_eq!(result, true)
}