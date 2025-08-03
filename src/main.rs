mod matcher;
mod parser;

use crate::matcher::RegexMatcher;
use std::env;
use std::io;
use std::process;

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

#[test]
fn start_of_string() {
    let input_text = "log";
    let pattern = "^log";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn end_of_string_01() {
    let input_text = "dog";
    let pattern = "dog$";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn end_of_string_02() {
    let input_text = "dog";
    let pattern = "^dog$";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn end_of_string_03() {
    let input_text = "dog";
    let pattern = "[fg]$";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn end_of_string_04() {
    let input_text = "dogdogdog";
    let pattern = "o[fg]$";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn one_or_more_01() {
    let input_text = "caat";
    let pattern = "ca+t";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn one_or_more_02() {
    let input_text = "caats";
    let pattern = "ca+at";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn one_or_more_03() {
    let input_text = "ca";
    let pattern = "ca+t";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, false);
}

#[test]
fn one_or_more_04() {
    let input_text = "cccccats";
    let pattern = "c+ats$";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn zero_or_one_01() {
    let input_text = "cats";
    let pattern = "c?ats";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn zero_or_one_02() {
    let input_text = "dogs";
    let pattern = "dogs?";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}

#[test]
fn zero_or_one_03() {
    let input_text = "dog";
    let pattern = "dogs?";

    let result = match_pattern(input_text, pattern);
    assert_eq!(result, true);
}
