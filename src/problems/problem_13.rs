use std::collections::VecDeque;

use crate::{input::input::InputParser, utility::parser::Parser};

pub fn solve_problem_13a(input: Vec<String>) -> usize {
    let pairs = PacketPart::parse_packet_pairs(input);
    pairs.into_iter().enumerate().map(
        |(i, (left, right))| 
            if PacketPart::is_ordered(&left, &right).unwrap() { i + 1 } else {0}
        ).sum()
}

fn solve_problem_13b(input: Vec<String>) -> usize {
    unimplemented!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketPart {
    List(Vec<PacketPart>),
    N(u32)
}

impl PacketPart {

    pub fn parse_packet_pairs(strings: Vec<String>) -> Vec<(Self, Self)> {
        InputParser::chunk(
            strings.into_iter().filter(|l| l.chars().count() > 0).collect::<Vec<_>>(),
            2
        ).expect("The input should be in pairs separated by whitespace.")
        .into_iter()
        .map(|pair| (Self::from_string(pair[0].clone()), Self::from_string(pair[1].clone())))
        .collect()
    }

    pub fn from_string(s: String) -> Self {
        let mut parser = Parser::new(s);
        let tokens = Self::tokenize(parser);
        Self::from_tokens(tokens).0
    }

    fn tokenize(mut parser: Parser) -> VecDeque<Token> {
        let mut to_return = VecDeque::new();
        while let Some(token) = Self::get_next_token(&mut parser) {
            to_return.push_back(token)
        }
        return to_return;
    }

    fn get_next_token(parser: &mut Parser) -> Option<Token> {
        if parser.n_remaining_to_parse() == 0 {
            return None;
        }
        let token = match parser.peek_char().unwrap() {
            '[' => {
                let _left_bracket = parser.consume_n(1);
                Token::LeftBracket
            },
            ']' => {
                let _right_bracket = parser.consume_n(1);
                if parser.peek_char() == Some(',') {
                    let _comma = parser.consume_n(1);
                }
                Token::RightBracket
            },
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Self::tokenize_number(parser)
            },
            c => panic!("Unexpected character: {}", c)
        };
        return Some(token);
    }

    fn tokenize_number(parser: &mut Parser) -> Token {
        let s = parser.consume_until(&|c| c == ',' || c == ']');
        let to_return = Token::N(s.parse::<u32>().expect(&format!("Number should be parsable into a u32 but was {}", s)));
        if parser.peek_char() == Some(',') {
            let _comma = parser.consume_n(1);
        }
        return to_return;
    }

    fn from_tokens(mut tokens: VecDeque<Token>) -> (Self, VecDeque<Token>) {
        match tokens.pop_front().expect("There should be at least one element in tokens.") {
            Token::N(n) => (Self::N(n), tokens),
            Token::LeftBracket => {
                let mut to_return = Vec::new();
                while tokens.get(0) != Some(&Token::RightBracket) {
                    let (parsed, new_tokens) = Self::from_tokens(tokens);
                    to_return.push(parsed);
                    tokens = new_tokens;
                }
                let right_bracket = tokens.pop_front();
                assert_eq!(right_bracket, Some(Token::RightBracket));
                return (Self::List(to_return), tokens);
            },
            Token::RightBracket => panic!("Got a right bracket when parsing.")
        }
    }

    pub fn is_ordered(left: &Self, right: &Self) -> Option<bool> {
        match (left, right) {
            (Self::N(l), Self::N(r)) => {
                if l == r {
                    return None;
                } else {
                    return Some(l < r);
                }
            },
            (Self::N(l), Self::List(r)) => {
                return Self::is_ordered(&Self::List(vec![Self::N(*l)]), right);
            },
            (Self::List(l), Self::N(r)) => {
                return Self::is_ordered(left, &Self::List(vec![Self::N(*r)]));
            },
            (Self::List(l), Self::List(r)) => {
                for (i, j) in l.iter().zip(r) {
                    match Self::is_ordered(i, j) {
                        None => {},
                        Some(x) => {return Some(x);}
                    };
                }
                if l.len() == r.len() {
                    return None;
                }
                return Some(l.len() < r.len());
            }
        }
    }
    
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Token {
    LeftBracket,
    RightBracket,
    N(u32),
}

#[cfg(test)]
mod test_problem_13 {

    use super::*;

    #[test]
    fn test_problem_13a_passes() {
        
        let example_input = InputParser::new().parse_as_string("example_input_13.txt").unwrap();
        assert_eq!(solve_problem_13a(example_input), 13);

        let input = InputParser::new().parse_as_string("input_13.txt").unwrap();

        let answer = solve_problem_13a(input);
        assert_eq!(answer, 5340);
    }
    
    #[test]
    fn test_problem_13b_passes() {
        let input = InputParser::new().parse_as_string("input_13.txt").unwrap();

        let answer = solve_problem_13b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_parses_correctly() {

        let example_input = InputParser::new().parse_as_string("example_input_13.txt").unwrap();

        let packet_pairs = PacketPart::parse_packet_pairs(example_input);
        assert_eq!(packet_pairs.len(), 8);
        assert_eq!(packet_pairs[5], (PacketPart::List(vec![]), PacketPart::List(vec![PacketPart::N(3)])));
        
        assert_eq!(
            packet_pairs[1],
            (
                PacketPart::List(vec![
                    PacketPart::List(vec![PacketPart::N(1)]),
                    PacketPart::List(vec![
                        PacketPart::N(2),
                        PacketPart::N(3),
                        PacketPart::N(4),
                    ])
                ]),
                PacketPart::List(vec![
                    PacketPart::List(vec![PacketPart::N(1)]),
                    PacketPart::N(4)
                ])
            )
        );
        
    }

    #[test]
    fn test_tokenizes_correctly() {
        assert_eq!(PacketPart::tokenize(Parser::new("1".to_string())), vec![Token::N(1)]);
        assert_eq!(PacketPart::tokenize(Parser::new("0190".to_string())), vec![Token::N(190)]);
        assert_eq!(PacketPart::tokenize(Parser::new("[190]".to_string())), vec![Token::LeftBracket, Token::N(190), Token::RightBracket]);
        assert_eq!(PacketPart::tokenize(Parser::new("[1,2,3]".to_string())), vec![Token::LeftBracket, Token::N(1), Token::N(2), Token::N(3), Token::RightBracket]);
    }

}