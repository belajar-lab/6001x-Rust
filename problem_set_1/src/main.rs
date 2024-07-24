use problem_set_1::{count_bob, count_vowels, longest_substring};

fn main() {
    let s = "azcbobobegghakl";
    println!("s = {}", s);

    // Problem 1
    println!("Number of vowels: {}", count_vowels(s));

    // Problem 2
    println!("Number of times bob occurs is: {}", count_bob(s));

    // Problem 3
    println!("Longest substring in alphabetical order is: {}", longest_substring(s));
}