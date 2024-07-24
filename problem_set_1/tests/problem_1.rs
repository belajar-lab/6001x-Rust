use problem_set_1::count_vowels;

#[test]
fn count_vowels_tests() {
    assert_eq!(count_vowels("hetseoxbadk"), 4);
    assert_eq!(count_vowels("numejfiqjajlfdihyhfzuoo"), 8);
    assert_eq!(count_vowels("owgseupxbp"), 3);
    assert_eq!(count_vowels("kuvabhxwhaqhspsddakpwow"), 5);
    assert_eq!(count_vowels("agurzqshtcbxnm"), 2);
    assert_eq!(count_vowels("hyueqpacbbohn"), 4);
    assert_eq!(count_vowels("ytaaubrvxjaoq"), 5);
    assert_eq!(count_vowels("youoeoiesjunfuxyocmmqx"), 10);
    assert_eq!(count_vowels("dybmaa"), 2);
    assert_eq!(count_vowels("rlaadtsynfuoroqxxegh"), 6);
    assert_eq!(count_vowels("payideo"), 4);
    assert_eq!(count_vowels("nnyaaasmthqeygill"), 5);
    assert_eq!(count_vowels("thaauqufhrz"), 4);
    assert_eq!(count_vowels("lvousenubil"), 5);
    assert_eq!(count_vowels("vnckialhxeumsenqdiw"), 6);
    assert_eq!(count_vowels("gbmhhezwctmhzaihw"), 3);
    assert_eq!(count_vowels("wacqagqfbdqgeuytu"), 5);
    assert_eq!(count_vowels("xsiedrejauowtql"), 6);
    assert_eq!(count_vowels("cozftregup"), 3);
    assert_eq!(count_vowels("otqitwoccqlvuio"), 6);
}