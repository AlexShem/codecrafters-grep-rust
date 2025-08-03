use crate::parser::{AnchorType, Pattern, PatternElement, Quantifier, RegexParser};

pub struct RegexMatcher {
    pattern: Pattern,
}

impl RegexMatcher {
    pub fn new(pattern: Pattern) -> Self {
        RegexMatcher { pattern }
    }

    pub fn compile_regex(pattern: &str) -> anyhow::Result<RegexMatcher> {
        let mut parser = RegexParser::new(pattern);
        let parsed_pattern = parser.parse()?;
        Ok(RegexMatcher::new(parsed_pattern))
    }

    pub fn is_match(&self, text: &str) -> bool {
        // Handle fully-anchored pattern: `^...$`
        if let (Some(first), Some(last)) =
            (self.pattern.elements.first(), self.pattern.elements.last())
        {
            if matches!(first, PatternElement::Anchor(AnchorType::StartOfString))
                && matches!(last, PatternElement::Anchor(AnchorType::EndOfString))
            {
                return self.match_at_position(text, 0);
            }
        }

        // Handle patterns that start with '^'
        if let Some(PatternElement::Anchor(AnchorType::StartOfString)) =
            self.pattern.elements.first()
        {
            return self.match_at_position(text, 0);
        }

        // Handle patterns that end with '$'
        if let Some(PatternElement::Anchor(AnchorType::EndOfString)) = self.pattern.elements.last()
        {
            for start_pos in 0..=text.len() {
                if self.match_at_position(text, start_pos) {
                    return true;
                }
            }
            return false;
        }

        for start_pos in 0..=text.len() {
            if self.match_at_position(text, start_pos) {
                return true;
            }
        }
        false
    }

    fn match_at_position(&self, text: &str, start_pos: usize) -> bool {
        let chars: Vec<char> = text.chars().collect();

        if start_pos > chars.len() {
            return self.pattern.elements.is_empty();
        }

        self.match_elements_at_position(&chars, start_pos, 0)
    }

    /// Recursively match pattern elements against text characters
    /// - `chars`: the text as a vector of characters
    /// - `text_pos`: current position in the text
    /// - `pattern_pos`: current position in the pattern
    fn match_elements_at_position(
        &self,
        chars: &Vec<char>,
        text_pos: usize,
        patter_pos: usize,
    ) -> bool {
        if patter_pos >= self.pattern.elements.len() {
            return true;
        }

        let current_element = &self.pattern.elements[patter_pos];

        // Handle anchor element
        if current_element.is_anchor() {
            return if current_element.check_anchor(text_pos, chars.len()) {
                self.match_elements_at_position(chars, text_pos, patter_pos + 1)
            } else {
                false
            };
        }

        // Handle quantified elements with special repetition logic
        if current_element.is_quantified() {
            return self.match_quantified_element(chars, text_pos, patter_pos);
        }

        // Reached the end of text but still have non-anchor pattern element - fail
        if text_pos >= chars.len() {
            return false;
        }

        let current_char = chars[text_pos];

        if current_element.matches_char(current_char) {
            self.match_elements_at_position(chars, text_pos + 1, patter_pos + 1)
        } else {
            false
        }
    }

    fn match_quantified_element(
        &self,
        chars: &Vec<char>,
        text_pos: usize,
        pattern_pos: usize,
    ) -> bool {
        let current_element = &self.pattern.elements[pattern_pos];

        if let PatternElement::Quantified {
            element,
            quantifier,
        } = current_element
        {
            match quantifier {
                Quantifier::Plus => {
                    // At least one match is expected
                    self.match_plus_quantifier(chars, text_pos, pattern_pos, element)
                }
            }
        } else {
            panic!("match_quantified_element called on non-quantified element")
        }
    }

    fn match_plus_quantifier(
        &self,
        chars: &Vec<char>,
        text_pos: usize,
        pattern_pos: usize,
        element: &PatternElement,
    ) -> bool {
        if text_pos >= chars.len() {
            return false;
        }

        if !element.matches_char(chars[text_pos]) {
            return false;
        }

        // Greedy algorithm: try to match as much as possible
        let mut current_pos = text_pos + 1;

        while current_pos < chars.len() && element.matches_char(chars[current_pos]) {
            current_pos += 1;
        }

        // Backtracking
        for try_from_pos in (text_pos + 1..=current_pos).rev() {
            if self.match_elements_at_position(chars, try_from_pos, pattern_pos + 1) {
                return true;
            }
        }

        false
    }
}
