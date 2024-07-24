pub fn count_vowels(s: &str) -> i32 {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let vowels_count = s.chars()
        .filter(|&c| vowels.contains(&c.to_ascii_lowercase()))
        .count(); 

    vowels_count as i32
}

pub fn count_bob(s: &str) -> i32 {
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

    counter
}

pub fn longest_substring(s: &str) -> String {
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

    top
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn problem_1_tests() {
        // assert_eq!(result, 4);
    }
}