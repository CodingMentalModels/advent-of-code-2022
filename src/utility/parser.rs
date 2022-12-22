use std::collections::HashSet;

#[derive(Debug)]
pub struct Parser {
    to_parse: String,
    pointer: usize,
}

impl Parser {

    pub fn new(commands: String) -> Self {
        Self { to_parse: commands, pointer: 0 }
    }

    pub fn n_remaining_to_parse(&self) -> usize {
        self.to_parse.len() - self.pointer
    }

    pub fn increment_pointer(&mut self, n: usize) {
        let new_pointer_value = self.pointer + n;
        if new_pointer_value > self.to_parse.len() {
            self.pointer = self.to_parse.len();
        } else {
            self.pointer = new_pointer_value;
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        self.to_parse.chars().nth(self.pointer)
    }

    pub fn consume_line(&mut self) -> String {
        for (i, c) in self.to_parse.chars().skip(self.pointer).enumerate() {
            if c == '\n' {
                let to_return = self.consume_n(i);
                let _new_line = self.consume_n(1);
                return to_return;
            }
        }
        self.consume_remaining()
    }

    pub fn consume_until(&mut self, condition: &dyn Fn(char) -> bool) -> String {
        for (i, c) in self.to_parse.chars().skip(self.pointer).enumerate() {
            if condition(c) {
                let to_return = self.consume_n(i);
                return to_return;
            }
        }
        self.consume_remaining()
    }

    pub fn consume_whitespace(&mut self) -> String {
        self.consume_until(&|ch| !Self::whitespace_chars().contains(&ch))
    }

    pub fn consume_until_whitespace(&mut self) -> String {
        self.consume_until(&|ch| Self::whitespace_chars().contains(&ch))
    }

    fn whitespace_chars() -> HashSet<char> {
        vec![' ', '\t', '\n'].into_iter().collect::<HashSet<_>>()
    }

    pub fn consume_remaining(&mut self) -> String {
        self.consume_n(self.n_remaining_to_parse())
    }

    pub fn consume_n(&mut self, n: usize) -> String {
        let to_return = if self.pointer + n > self.to_parse.len() {
            self.to_parse[self.pointer..self.to_parse.len()].to_string()
        } else {
            self.to_parse[self.pointer..(self.pointer + n)].to_string()
        };
        self.increment_pointer(n);
        return to_return;
    }

    pub fn expect_end_of_line_or_file(&mut self) -> Result<String, String> {
        if self.pointer == self.to_parse.len() {
            return Ok("".to_string());
        }
        self.expect_end_of_line().map_err(|_| format!("Expected end of file but got: {}", self.consume_remaining()))
    }

    pub fn expect_end_of_line(&mut self) -> Result<String, String> {
        self.expect("\n")
    }

    pub fn expect(&mut self, s: &str) -> Result<String, String> {
        let length = s.len();
        let result = self.consume_n(length);
        if result == s {
            return Ok(result);
        } else {
            return Err(format!("Expected {}, but got {}", s, result));
        }
    }

}
