fn main() {
    let s = "azcbobobegghakl";

    let vowels = ["a", "e", "i", "o", "u"];
    let mut vowels_count = 0;

    for c in s.chars() {
        if vowels.contains(c.to_lowercase().next().unwrap()) {
            vowels_count += 1;
        }
    }

    println!("Number of vowels: {}", vowels_count);
}
