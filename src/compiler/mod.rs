use std::str::Chars;
use std::iter::Peekable;

pub mod token;
pub mod parser;

use compiler::token::Token;

pub struct Scanner<'a> {
    line: usize,
    source: Peekable<Chars<'a>>
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

impl<'a> Scanner<'a> {

    pub fn new(input: &str) -> Scanner {
        Scanner {
            line: 0,
            source: input.chars().peekable()
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.source.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn peek_match(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&p_c) => p_c == ch,
            None => false
        }
    }

    fn skip_comment(&mut self) {
        loop {
            match self.peek_char() {
                Some(&c) => {
                    if c == '\n' {
                        self.read_char();
                        self.line += 1;

                        break;
                    }
                    self.read_char();
                },
                None => {
                    self.line += 1;

                    break;
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if !ch.is_whitespace() {
                break;
            }
            let c = self.read_char();
            if c == Some('\n') {
                self.line += 1;
            }
        }
    }

    fn peek_alpha(&mut self) -> bool {
        match self.peek_char() {
            Some(&c) => is_letter(c),
            None => false
        }
    }

    fn peek_digit(&mut self) -> bool {
        match self.peek_char() {
            Some(&c) => c.is_numeric(),
            None => false
        }
    }

    fn read_word(&mut self, first: char) -> String {
        let mut s = String::new();
        s.push(first);

        while self.peek_alpha() || self.peek_digit() {
            s.push(self.read_char().unwrap());
        }

        return s
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut s = String::new();
        s.push(first);

        while self.peek_digit() {
            s.push(self.read_char().unwrap());
        }

        // Check whether we're dealing with floating point

        if self.peek_char() == Some(&'.') {
            s.push(self.read_char().unwrap());

            while self.peek_digit() {
                s.push(self.read_char().unwrap());
            }
            return Token::FloatLiteral(s.parse().expect("Invalid floating pt number"))
        }
        return Token::IntegerLiteral(s.parse().expect("Invalid number"))
    }

    fn read_string(&mut self) -> Token {
        let mut s = String::new();

        loop {
            if self.peek_match('"') {
                self.read_char();
                break;
            } else {
            let c = self.read_char();
                match Some(c) {
                    None => break,
                    _ => s.push(c.unwrap()),
                }
            }
        }
        return Token::StringLiteral(s)
    }

    fn skip(&mut self, num: usize) {
        let mut i = num;
        loop {
            self.source.next();

            i -= 1;
            if i == 0 {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {

            Some('+') => Token::Add,
            Some('-') => Token::Subtract,
            Some('*') => Token::Multiply,
            Some('/') => {
                if self.peek_match('/') {
                    self.skip_comment();
                    Token::Comment
                } else {
                    Token::Divide
                }
            },

            Some(',') => Token::Comma,
            Some('.') => {
                if self.peek_match('.') {
                    self.read_char();
                    return Token::DotDot
                }
                Token::Dot
            },

            Some('(') => Token::LeftParenthesis,
            Some(')') => Token::RightParenthesis,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,

            Some(';') => Token::Semicolon,

            Some(':') => Token::Colon,

            Some('"') => self.read_string(),

            Some('^') => Token::Xor,

            Some('%') =>Token::Modulo,

            Some('|') => {
                if self.peek_match('|') {
                    self.read_char();
                    Token::LogicalOr
                } else {
                    Token::Or
                }
            },

            Some('&') => {
                if self.peek_match('&') {
                    self.read_char();
                    Token::LogicalAnd
                } else {
                    Token::And
                }
            },

            Some('=') => {
                if self.peek_match('=') {
                    self.read_char();
                    Token::Equality
                } else {
                    Token::Assign
                }
            },

            Some('!') => {
                if self.peek_match('=') {
                    self.read_char();
                    Token::NotEquality
                } else {
                    Token::Bang
                }
            },

            Some('<') => {
                if self.peek_match('=') {
                    self.read_char();
                    Token::LessThanEqual
                } else if self.peek_match('<') {
                    self.read_char();
                    Token::ShiftLeft
                } else {
                    Token::LessThan
                }
            },

            Some('>') => {
                if self.peek_match('=') {
                    self.read_char();
                    Token::GreaterThanEqual
                } else if self.peek_match('>') {
                    self.read_char();
                    Token::ShiftRight
                } else {
                    Token::GreaterThan
                }
            },

            Some(ch @ _) => {
                if is_letter(ch) {
                    let ident = self.read_word(ch);
                    token::lookup(&ident)
                } else if ch.is_numeric() {
                    self.read_number(ch)
                } else {
                    println!("Error at line: {}", self.line);
                    Token::Illegal
                }
            },

            None => Token::EOF

        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_scanner<'a>() -> Scanner<'a> {
        let test_scanner = Scanner::new("ab12\na 123");

        return test_scanner;
    }

    #[test]
    fn test_read_char() {
        let mut test_scanner = get_test_scanner();

        assert_eq!(test_scanner.read_char(), Some('a'));
    }

    #[test]
    fn test_peek_char() {
        let mut test_scanner = get_test_scanner();

        assert_eq!(test_scanner.peek_char(), Some(&'a'));
    }

    #[test]
    fn test_peek_match() {
        let mut test_scanner = get_test_scanner();

        assert_eq!(test_scanner.peek_match('a'), true);
    }

    #[test]
    fn test_peek_alpha() {
        let mut test_scanner = get_test_scanner();

        assert_eq!(test_scanner.peek_alpha(), true);
    }

    #[test]
    fn test_peek_number() {
        let mut test_scanner = get_test_scanner();

        test_scanner.read_char();
        test_scanner.read_char();

        assert_eq!(test_scanner.peek_digit(), true);
    }

    #[test]
    fn test_skip_whitespace() {
        let mut test_scanner = get_test_scanner();

        test_scanner.skip(4);

        test_scanner.skip_whitespace();

        assert_eq!(test_scanner.peek_char(), Some(&'a'));
        assert_eq!(test_scanner.line, 1);
    }
}
