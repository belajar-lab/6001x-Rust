fn problem_1() {
    let s = "azcbobobegghakl";

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let vowels_count = s.chars()
        .filter(|&c| vowels.contains(&c.to_ascii_lowercase()))
        .count(); 

    println!("Number of vowels: {}", vowels_count);
}

fn problem_2() {
    let s = "azcbobobegghakl";

    let chars: Vec<char> = s.chars().collect();
    let mut counter = 0;
    let mut start = 0;
    let mut end = 3;

    while end <= chars.len() {
        if chars[start..end] == ['b', 'o', 'b'] {
            counter += 1;
        }
        start += 1;
        end += 1;
    }

    println!("Number of times bob occurs is: {}", counter);
}

fn problem_3() {
    let s = String::from("azcbobobegghakl");
    let mut top = String::new();
    let mut current = String::new();

    for c in s.chars() {
        if current.is_empty() || c >= current.chars().last().unwrap() {
            current.push(c);
        } else {
            if top.len() < current.len() {
                top = current;
            }
            current = c.to_string();
        }
    }

    // Final check after loop ends
    if top.len() < current.len() {
        top = current;
    }

    println!("Longest substring in alphabetical order is: {}", top);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn problem_1_tests() {
        // assert_eq!(result, 4);
    }
}