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
    use super::*;

    #[test]
    fn problem_1_tests() {
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
}