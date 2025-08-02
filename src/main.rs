use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    // Match `\d` any digit
    if pattern.to_string() == "\\d".to_string() {
        return input_line.chars().any(|c| c.is_ascii_digit());
    }

    // Match `\w` alphanumeric character
    if pattern.to_string() == "\\w".to_string() {
        return input_line.chars().any(|c| c.is_ascii_alphanumeric() || c == '_');
    }

    // Match `[]` character group
    if pattern.chars().nth(0).unwrap() == '[' {
        let mut is_positive = true;
        let mut group_chars: Vec<char> = Vec::new();
        let mut position: usize = 1;
        let pattern_chars: Vec<char> = pattern.chars().collect();

        // Check if the group is negative
        if position < pattern.len() && pattern_chars[position] == '^' {
            is_positive = false;
            position += 1;
        }

        while position < pattern.len() && pattern_chars[position] != ']' {
            group_chars.push(pattern_chars.get(position).copied().unwrap());
            position += 1;
        }
        if position < pattern_chars.len() && pattern_chars[position] == ']' {
            return if is_positive {
                input_line.chars().any(|c| group_chars.contains(&c))
            } else {
                input_line.chars().any(|c| !group_chars.contains(&c))
            };
        }
        return false;
    }

    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
#[test]
fn match_digits() {
    let input_text = "apple123";
    let pattern = "\\d";

    let result = match_pattern(input_text, pattern);
    assert!(result)
}

#[test]
fn positive_character_groups_01() {
    let input_text = "apple";
    let pattern = "[abc]";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true)
}

#[test]
fn positive_character_groups_02() {
    let input_text = "[]";
    let pattern = "[strawberry]";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, false)
}

#[test]
fn negative_character_groups_01() {
    let input_text = "dog";
    let pattern = "[^abc]";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true)
}

#[test]
fn negative_character_groups_02() {
    let input_text = "cab";
    let pattern = "[^abc]";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, false)
}