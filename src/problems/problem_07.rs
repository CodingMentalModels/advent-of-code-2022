use std::collections::{VecDeque, HashSet};

use crate::input::input::InputParser;

pub fn solve_problem_07a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_07b(input: Vec<String>) -> usize {
    unimplemented!();
}

enum FileTreeNode {
    File(String, usize),
    Directory(String, Vec<FileTreeNode>),
}

impl FileTreeNode {

    pub fn file(name: String, size: usize) -> Self {
        Self::File(name, size)
    }

    pub fn directory(name: String, children: Vec<Self>) -> Self {
        assert!(children.len() > 0);
        Self::Directory(name, children)
    }

    pub fn len(&self) -> usize {
        match self {
            Self::File(_, _) => { 1 },
            Self::Directory(_, children) => { 1 + children.iter().map(|c| c.len()).sum::<usize>() }
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::File(_, _) => { 1 },
            Self::Directory(_, children) => { 1 + children.iter().map(|c| c.depth()).max().expect("All branches should have children") }
        }
    }

    pub fn sum_sizes_if(&self, f: fn(usize) -> bool) -> usize {
        match self {
            Self::File(_, size) => { if f(*size) {*size} else {0} },
            Self::Directory(_, children) => { children.iter().map(|c| c.sum_sizes_if(f)).sum() }
        }
    }

    pub fn sum_sizes(&self) -> usize {
        self.sum_sizes_if(|n| true)
    }
    
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CommandStream {
    commands: VecDeque<Command>,
}

impl CommandStream {

    pub fn new(commands: VecDeque<Command>) -> Self {
        Self {commands}
    }

    pub fn from_string(commands: String) -> Result<Self, String> {
        let mut parser = CommandParser::new(commands);
        parser.parse()
    }

    pub fn pop(&mut self) -> Option<Command> {
        self.commands.pop_front()
    }

}

#[derive(Debug)]
struct CommandParser {
    to_parse: String,
    pointer: usize,
}

impl CommandParser {

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
    pub fn parse(&mut self) -> Result<CommandStream, String> {
        let mut to_return = VecDeque::new();
        while self.n_remaining_to_parse() > 0 {
            to_return.push_back(self.parse_command()?);
        }
        return Ok(CommandStream::new(to_return));
    }

    fn parse_command(&mut self) -> Result<Command, String> {
        let _dollar_sign = self.expect("$ ")?;
        match self.peek_char() {
            Some('l') => {
                let ls = self.expect("ls");
                self.expect_end_of_line_or_file()?;
                let contents: Result<Vec<CommandResult>, String> = self.consume_until_next_command().into_iter().map(|c| Self::parse_command_result(c)).collect();
                return Ok(Command::Ls(contents?));
            },
            Some('c') => {
                let cd = self.expect("cd ");
                let directory = self.consume_until_whitespace();
                self.expect_end_of_line_or_file()?;
                return Ok(Command::Cd(directory));
            },
            c => return Err(format!("Unexpected character found: {:?}.\n\nParser: {:?}", c, self)),
        }
    }

    fn parse_command_result(s: String) -> Result<CommandResult, String> {
        let components = s.split_whitespace().collect::<Vec<_>>();
        if components.len() != 2 {
            return Err(format!("Expected two components of a command result but got {:?}", components));
        }
        match s.chars().nth(0) {
            Some('d') => {
                let (_dir_string, name_string) = (components[0], components[1]);
                return Ok(CommandResult::Dir(name_string.to_string()));
            },
            Some(_) => {
                let (size_string, name_string) = (components[0], components[1]);

                return Ok(CommandResult::File(name_string.to_string(), size_string.parse::<usize>().map_err(|_| "Unable to parse size".to_string())?));
            },
            None => {
                return Err("Tried to parse a string with no characters!".to_string());
            }
        }
    }
    fn peek_char(&self) -> Option<char> {
        self.to_parse.chars().nth(self.pointer)
    }

    fn consume_line(&mut self) -> String {
        for (i, c) in self.to_parse.chars().skip(self.pointer).enumerate() {
            if c == '\n' {
                let to_return = self.consume_n(i);
                let _new_line = self.consume_n(1);
                return to_return;
            }
        }
        self.consume_remaining()
    }

    fn consume_until_next_command(&mut self) -> Vec<String> {
        let mut to_return = Vec::new();
        while let Some(next_char) = self.peek_char() {
            if next_char == '$' {
                return to_return;
            } else {
                to_return.push(self.consume_line())
            }
        }
        return to_return;
    }
    
    fn consume_until_whitespace(&mut self) -> String {
        let whitespace_chars = vec![' ', '\t', '\n'].into_iter().collect::<HashSet<_>>();
        for (i, c) in self.to_parse.chars().skip(self.pointer).enumerate() {
            if whitespace_chars.contains(&c) {
                let to_return = self.consume_n(i);
                return to_return;
            }
        }
        self.consume_remaining()
    }

    fn consume_remaining(&mut self) -> String {
        self.consume_n(self.n_remaining_to_parse())
    }

    fn consume_n(&mut self, n: usize) -> String {
        let to_return = if self.pointer + n > self.to_parse.len() {
            self.to_parse[self.pointer..self.to_parse.len()].to_string()
        } else {
            self.to_parse[self.pointer..(self.pointer + n)].to_string()
        };
        self.increment_pointer(n);
        return to_return;
    }

    fn expect_end_of_line_or_file(&mut self) -> Result<String, String> {
        if self.pointer == self.to_parse.len() {
            return Ok("".to_string());
        }
        self.expect_end_of_line()
    }

    fn expect_end_of_line(&mut self) -> Result<String, String> {
        self.expect("\n")
    }

    fn expect(&mut self, s: &str) -> Result<String, String> {
        let length = s.len();
        let result = self.consume_n(length);
        if result == s {
            return Ok(result);
        } else {
            return Err(format!("Expected {}, but got {}", s, result));
        }
    }

}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Cd(String),
    Ls(Vec<CommandResult>),
}

impl Command {

}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CommandResult {
    Dir(String),
    File(String, usize),
}

#[cfg(test)]
mod test_problem_07 {

    use super::*;

    #[test]
    fn test_problem_07a_passes() {
        
        let example = InputParser::new().parse_as_string("example_input_07.txt").unwrap();

        let answer = solve_problem_07a(example);
        assert_eq!(answer, 95437);

        let input = InputParser::new().parse_as_string("input_07.txt").unwrap();

        let answer = solve_problem_07a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_07b_passes() {
        let input = InputParser::new().parse_as_string("input_07.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_07b(shorted_input), 0);

        let answer = solve_problem_07b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_filetree_constructs() {
        let children = (1..4).into_iter().map(|i| FileTreeNode::file(format!("Child_{}", i), i)).collect();
        let tree = FileTreeNode::directory("tree".to_string(), children);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.sum_sizes(), 6);
        assert_eq!(tree.sum_sizes_if(|size| size <= 2), 3);
    }

    #[test]
    fn test_command_stream_parses_commands() {
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
        ];
        let mut stream = CommandStream::from_string(input.join("\n")).unwrap();

        assert_eq!(stream.pop(), Some(Command::Cd("/".to_string())));
        assert_eq!(
            stream.pop(), Some(Command::Ls(
                vec![
                    CommandResult::Dir("a".to_string()),
                    CommandResult::File("b.txt".to_string(), 14848514),
                    CommandResult::File("c.dat".to_string(), 8504156),
                    CommandResult::Dir("d".to_string()),
                ]
            ))
        );
        assert_eq!(stream.pop(), Some(Command::Cd("a".to_string())));
    }

    #[test]
    fn test_parser_peeks() {
        
        let mut parser = CommandParser::new("awpoeiht".to_string());
        assert_eq!(parser.peek_char(), Some('a'));
        assert_eq!(parser.pointer, 0);

        parser.consume_remaining();
        assert_eq!(parser.peek_char(), None);
    }

    #[test]
    fn test_parser_consumes_until_next_command() {
        
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
        ];

        let mut parser = CommandParser::new(input.join("\n"));
        let empty = parser.consume_until_next_command();
        assert_eq!(empty.len(), 0);

        parser.consume_line();
        parser.consume_line();
        let results = parser.consume_until_next_command();
        assert_eq!(results.len(), 4, "{:?}", results);
        assert_eq!(results[0], "dir a".to_string());

    }

    #[test]
    fn test_parser_parses_command_results() {
        
        let input = vec![
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
        ];

        let expected = vec![
            CommandResult::Dir("a".to_string()),
            CommandResult::File("b.txt".to_string(), 14848514),
            CommandResult::File("c.dat".to_string(), 8504156),
            CommandResult::Dir("d".to_string()),
        ];

        assert_eq!(input.into_iter().map(|c| CommandParser::parse_command_result(c).unwrap()).collect::<Vec<_>>(), expected);
    }
    

    #[test]
    fn test_command_parser_parses() {
        
        let input = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
        ];
        let mut parser = CommandParser::new(input.join("\n"));
        assert_eq!(parser.parse_command().unwrap(), Command::Cd("/".to_string()));
    }
}
