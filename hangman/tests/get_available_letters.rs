use hangman::get_available_letters;

#[test]
fn test_1(){
    let letters_guessed = vec!['e', 'i', 'k', 'p', 'r', 's'];
    let result = get_available_letters(&letters_guessed);
    assert_eq!(result, "abcdfghjlmnoqtuvwxyz".to_string())
}

#[test]
fn test_2(){
    let letters_guessed = vec![];
    let result = get_available_letters(&letters_guessed);
    assert_eq!(result, "abcdefghijklmnopqrstuvwxyz".to_string())
}

#[test]
fn random_test_1(){
    let letters_guessed = vec!['b'];
    let result = get_available_letters(&letters_guessed);
    assert_eq!(result, "acdefghijklmnopqrstuvwxyz".to_string())
}

#[test]
fn random_test_2(){
    let letters_guessed = vec!['r', 'a', 'f'];
    let result = get_available_letters(&letters_guessed);
    assert_eq!(result, "bcdeghijklmnopqstuvwxyz".to_string())
}

#[test]
fn random_test_3(){
    let letters_guessed = vec!['m', 'i', 'd', 'c', 'z', 'h'];
    let result = get_available_letters(&letters_guessed);
    assert_eq!(result, "abefgjklnopqrstuvwxy".to_string())
}

#[test]
fn random_test_4(){
    let letters_guessed = vec!['s', 'i', 'f', 't', 'g', 'n', 'e', 'v', 'u', 'k', 'p', 'j'];
    let result = get_available_letters(&letters_guessed);
    assert_eq!(result, "abcdhlmoqrwxyz".to_string())
}