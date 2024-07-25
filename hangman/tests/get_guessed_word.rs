use hangman::get_guessed_word;

#[test]
fn test_1() {
    let secret_word = "apple";
    let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];

    let result = get_guessed_word(secret_word, &letters_guessed);
    assert_eq!(result, "_pp_e".to_string())
}

#[test]
fn test_2() {
    let secret_word = "durian";
    let letters_guessed = vec!['a', 'c', 'd', 'h', 'i', 'm', 'n', 'r', 't', 'u'];

    let result = get_guessed_word(secret_word, &letters_guessed);
    assert_eq!(result, "durian".to_string())
}

#[test]
fn random_test_1() {
    let secret_word = "banana";
    let letters_guessed = vec!['w', 'r', 'h', 'g', 't', 'f', 'p', 'y', 'o', 'q'];

    let result = get_guessed_word(secret_word, &letters_guessed);
    assert_eq!(result, "______".to_string())
}

#[test]
fn random_test_2() {
    let secret_word = "coconut";
    let letters_guessed = vec!['m', 'j', 'v', 'e', 'f', 'x', 'q', 'o', 'u', 't'];

    let result = get_guessed_word(secret_word, &letters_guessed);
    assert_eq!(result, "_o_o_ut".to_string())
}

#[test]
fn random_test_3() {
    let secret_word = "mangosteen";
    let letters_guessed = vec![];

    let result = get_guessed_word(secret_word, &letters_guessed);
    assert_eq!(result, "__________".to_string())
}

#[test]
fn random_test_4() {
    let secret_word = "broccoli";
    let letters_guessed = vec!['q', 'e', 'l', 's', 'b', 'h', 'd', 'p', 'n', 'x'];

    let result = get_guessed_word(secret_word, &letters_guessed);
    assert_eq!(result, "b_____l_".to_string())
}