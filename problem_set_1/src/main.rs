fn problem_1() {
    let s = "azcbobobegghakl";

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let vowels_count = s.chars()
        .filter(|&c| vowels.contains(&c.to_ascii_lowercase()))
        .count(); 

    println!("Number of vowels: {}", vowels_count);
}

fn main() {
    problem_1();
}