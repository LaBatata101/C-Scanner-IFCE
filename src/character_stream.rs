#[derive(Debug)]
pub struct CharacterStream {
    pos: usize,
    chars: Vec<char>,
    // current_char: char,
    is_end_of_file: bool
}

impl CharacterStream {
    pub fn new(text: &str) -> Self {
        let chars = text.chars().collect::<Vec<char>>();

        Self { pos: 0, chars, is_end_of_file: text.is_empty() }
    }

    pub fn current_char(&self) -> char {
        self.chars[self.pos]
    }

    pub fn next_char(&self) -> Option<&char> {
        self.chars.get(self.pos + 1)
    }

    pub fn advance_by(&mut self, offset: usize) {
        self.pos += offset;

        if self.pos >= self.chars.len() {
            self.is_end_of_file = true;
        }
    }

    pub fn is_eof(&self) -> bool {
        self.is_end_of_file
    }

    pub fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.current_char().is_ascii_whitespace() {
            self.advance_by(1)
        }
    }

    pub fn check_bounds(&self) -> bool {
        self.pos < self.chars.len()
    }
}
