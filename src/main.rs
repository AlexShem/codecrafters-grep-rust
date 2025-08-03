mod matcher;

use std::env;
use std::io;
use std::process;
use crate::matcher::RegexMatcher;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if let Ok(matcher) = RegexMatcher::compile_regex(pattern) {
        matcher.is_match(input_line)
    } else {
        false
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