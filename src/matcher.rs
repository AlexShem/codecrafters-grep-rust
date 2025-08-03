use anyhow::anyhow;

pub enum PatternElement {
    /// A literal character like 'a', 'b', 'c', ...
    Literal(char),

    /// Predefined character classes
    CharacterClass(CharClass),

    /// Custom character groups like `[abc]` or `[^abc]`.
    CharacterGroup { chars: Vec<char>, negated: bool },
}

pub enum CharClass {
    /// `\d` - matches digits 0-9
    Digit,

    /// `\w` - matches word characters (alphanumeric + `_` underscore)
    Word,
}

pub struct Pattern {
    pub elements: Vec<PatternElement>,
}

pub struct RegexMatcher {
    pattern: Pattern,
}

struct RegexParser {
    input: Vec<char>,
    position: usize,
}

impl PatternElement {
    fn matches_char(&self, ch: char) -> bool {
        match self {
            PatternElement::Literal(literal_ch) => *literal_ch == ch,
            PatternElement::CharacterClass(char_class) => char_class.matches_char(ch),
            PatternElement::CharacterGroup { chars, negated } => {
                let contains = chars.contains(&ch);
                if *negated { !contains } else { contains }
            }
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

    fn match_elements_at_position(
        &self,
        chars: &Vec<char>,
        text_pos: usize,
        patter_pos: usize,
    ) -> bool {
        if patter_pos >= self.pattern.elements.len() {
            return true;
        }

        if text_pos >= chars.len() {
            return false;
        }

        let current_element = &self.pattern.elements[patter_pos];
        let current_char = chars[text_pos];

        if current_element.matches_char(current_char) {
            self.match_elements_at_position(chars, text_pos + 1, patter_pos + 1)
        } else {
            false
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

    pub fn parse(&mut self) -> anyhow::Result<Pattern> {
        let mut elements = Vec::new();

        while self.position < self.input.len() {
            elements.push(self.parse_element()?);
        }

        Ok(Pattern { elements })
    }

    fn parse_element(&mut self) -> anyhow::Result<PatternElement> {
        let ch = self.current_char()?;

        match ch {
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