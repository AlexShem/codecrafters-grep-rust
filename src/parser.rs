use anyhow::anyhow;

pub enum PatternElement {
    /// A literal character like 'a', 'b', 'c', ...
    Literal(char),

    /// Predefined character classes
    CharacterClass(CharClass),

    /// Custom character groups like `[abc]` or `[^abc]`.
    CharacterGroup { chars: Vec<char>, negated: bool },

    /// Position anchors that do not consume characters
    Anchor(AnchorType),

    /// Quantified element. Contains:
    /// - `element`: `Box<PatternElement>`
    /// - `quantifier`: `Quantifier`
    Quantified {
        element: Box<PatternElement>,
        quantifier: Quantifier,
    },
}

pub enum Quantifier {
    /// `+` matches one or more times
    Plus,
}

pub enum CharClass {
    /// `\d` - matches digits 0-9
    Digit,

    /// `\w` - matches word characters (alphanumeric + `_` underscore)
    Word,
}

pub enum AnchorType {
    /// `^` matches the beginning of the string
    StartOfString,

    /// `$` matches the end of the string
    EndOfString,
}

pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

pub struct RegexParser {
    input: Vec<char>,
    position: usize,
}

impl PatternElement {
    pub fn matches_char(&self, ch: char) -> bool {
        match self {
            PatternElement::Literal(literal_ch) => *literal_ch == ch,
            PatternElement::CharacterClass(char_class) => char_class.matches_char(ch),
            PatternElement::CharacterGroup { chars, negated } => {
                let contains = chars.contains(&ch);
                if *negated {
                    !contains
                } else {
                    contains
                }
            }
            PatternElement::Anchor(_) => {
                panic!("'matches_char' called on anchor element - use check_anchor instead")
            }
            PatternElement::Quantified { element, .. } => element.matches_char(ch),
        }
    }

    pub fn is_anchor(&self) -> bool {
        matches!(self, PatternElement::Anchor(_))
    }

    pub fn is_quantified(&self) -> bool {
        matches!(self, PatternElement::Quantified { .. })
    }

    pub fn check_anchor(&self, text_pos: usize, text_len: usize) -> bool {
        match self {
            PatternElement::Anchor(AnchorType::StartOfString) => text_pos == 0,
            PatternElement::Anchor(AnchorType::EndOfString) => text_pos == text_len,
            _ => panic!("'check_anchor' called on non-anchor element"),
        }
    }
}

impl CharClass {
    fn matches_char(&self, ch: char) -> bool {
        match self {
            CharClass::Digit => ch.is_ascii_digit(),
            CharClass::Word => ch.is_ascii_alphanumeric() || ch == '_',
        }
    }
}

impl RegexParser {
    pub fn new(regex: &str) -> Self {
        RegexParser {
            input: regex.chars().collect(),
            position: 0,
        }
    }

    /// Returns the next character `Some(char)` in the input without consuming it.
    /// If there are no more characters, returns `None`.
    fn peek(&self) -> Option<char> {
        if self.position < self.input.len() {
            return Some(self.input[self.position]);
        }
        None
    }

    pub fn parse(&mut self) -> anyhow::Result<Pattern> {
        let mut elements = Vec::new();

        while self.position < self.input.len() {
            let mut element = self.parse_element()?;

            // Check if the next character is a quantifier
            if let Some(next_char) = self.peek() {
                if let Some(quantifier) = self.parse_quantifier(next_char) {
                    self.advance();
                    element = PatternElement::Quantified {
                        element: Box::new(element),
                        quantifier,
                    }
                }
            }

            elements.push(element);
        }

        Ok(Pattern { elements })
    }

    pub fn parse_quantifier(&self, ch: char) -> Option<Quantifier> {
        match ch {
            '+' => Some(Quantifier::Plus),
            '?' => todo!("'?' Is not yet supported as a quantifier"),
            _ => None,
        }
    }

    fn parse_element(&mut self) -> anyhow::Result<PatternElement> {
        let ch = self.current_char()?;

        match ch {
            '^' => {
                self.advance();
                Ok(PatternElement::Anchor(AnchorType::StartOfString))
            }
            '$' => {
                self.advance();
                Ok(PatternElement::Anchor(AnchorType::EndOfString))
            }
            '\\' => self.parse_escape_sequence(),
            '[' => self.parse_character_group(),
            _ => {
                self.advance();
                Ok(PatternElement::Literal(ch))
            }
        }
    }

    fn parse_escape_sequence(&mut self) -> anyhow::Result<PatternElement> {
        self.advance(); // Consume the backslash '\'
        let ch = self.current_char()?;
        self.advance(); // Go to the next character before returning the value

        match ch {
            'd' => Ok(PatternElement::CharacterClass(CharClass::Digit)),
            'w' => Ok(PatternElement::CharacterClass(CharClass::Word)),
            _ => Ok(PatternElement::Literal(ch)), // Escaped literal character (for some reason)
        }
    }

    fn parse_character_group(&mut self) -> anyhow::Result<PatternElement> {
        self.advance(); // Consume the opening bracket '['

        let negated = if self.current_char()? == '^' {
            self.advance();
            true
        } else {
            false
        };

        let mut chars = Vec::new();

        while self.position < self.input.len() && self.current_char()? != ']' {
            chars.push(self.current_char()?);
            self.advance();
        }

        if self.position >= self.input.len() {
            return Err(anyhow!("Unclosed character group"));
        }

        self.advance();
        Ok(PatternElement::CharacterGroup { chars, negated })
    }

    fn current_char(&self) -> anyhow::Result<char> {
        self.input
            .get(self.position)
            .copied()
            .ok_or_else(|| anyhow!("Unexpected end of input"))
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}
