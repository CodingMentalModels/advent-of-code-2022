use std::collections::VecDeque;

use crate::input::input::InputParser;

pub fn solve_problem_05a(input: String) -> String {
    let (mut stacks, instructions) = parse_input(input);
    
    stacks.handle_instructions(instructions, Model::Model9000);
    return stacks.get_top_boxes_string();
}

fn solve_problem_05b(input: String) -> String {
    let (mut stacks, instructions) = parse_input(input);
    
    stacks.handle_instructions(instructions, Model::Model9001);
    return stacks.get_top_boxes_string();
}

fn parse_input(input: String) -> (Stacks, Vec<Instruction>) {
    let chunks = input.split("\n\n").collect::<Vec<_>>();
    assert_eq!(chunks.len(), 2);
    
    let (stacks_string, instructions_string) = (chunks[0], chunks[1]);
    
    let stacks_string = stacks_string.split("\n").collect::<Vec<_>>();
    let l = stacks_string.len();
    let stacks_string: Vec<&str> = stacks_string.into_iter().take(l - 1).collect();
    let stacks_string = stacks_string.join("\n");
    let stacks = Stacks::from_string(stacks_string).expect("Unable to parse Stacks.");

    let instructions = instructions_string.split("\n")
        .map(|s| Instruction::from_string(s.to_string()).expect(&format!("Unable to parse instruction: {}", s)))
        .collect();
    
    return (stacks, instructions);
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {

    pub fn new(stacks: Vec<Vec<char>>) -> Self {
        Self {stacks}
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let row_boxes: Result<Vec<Vec<Option<char>>>, String> = s.split("\n").map(|row_string| Self::parse_row(row_string.to_string())).collect();
        let row_boxes: Vec<_> = row_boxes?.into_iter().rev().collect();
        assert!(row_boxes.len() > 0);
        let n_columns = row_boxes[0].len();
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_columns];
        for (_i, row) in row_boxes.into_iter().enumerate() {
            for (j, maybe_entry) in row.into_iter().enumerate() {
                match maybe_entry {
                    Some(entry) => {
                        stacks[j].push(entry);
                    },
                    None => {}
                }
            }
        }
        return Ok(Self::new(stacks));
    }

    fn parse_row(s: String) -> Result<Vec<Option<char>>, String> {
        let mut to_return = Vec::new();
        let mut chars = s.chars().collect::<VecDeque<_>>();
        while chars.len() > 0 {
            let maybe_left_bracket = chars.pop_front();
            let maybe_char = chars.pop_front();
            let maybe_right_bracket = chars.pop_front();
            let maybe_space = chars.pop_front();

            match maybe_left_bracket {
                Some(' ') => to_return.push(None),
                Some('[') => {
                    let c = match maybe_char {
                        None => {return Err("String ended prematurely.".to_string())},
                        Some(c) => c
                    };
                    to_return.push(Some(c));
                }
                c => return Err(format!("Got an unexpected character: {:?}", c))
            }
        }
        return Ok(to_return);
    }

    pub fn get_stack(&self, n: usize) -> &Vec<char> {
        &self.stacks[n]
    }

    pub fn get_n_boxes(&self) -> usize {
        self.stacks.iter().map(|c| c.len()).sum()
    }

    pub fn get_top_boxes(&self) -> Vec<char> {
        self.stacks.iter().map(|stack| *stack.last().unwrap_or(&' ')).collect()
    }

    pub fn get_top_boxes_string(&self) -> String {
        let s = self.get_top_boxes().into_iter().map(|c| c.to_string()).collect::<Vec<_>>().join("");
        return s.to_string()
    }

    pub fn handle_instructions(&mut self, instructions: Vec<Instruction>, model: Model) {
        instructions.into_iter().for_each(|i| self.handle_instruction(i, model))
    }

    pub fn handle_instruction(&mut self, instruction: Instruction, model: Model) {
        let (number_to_move, from_stack, to_stack) = instruction.unpack();
        match model {
            Model::Model9000 => {
                (0..number_to_move).for_each(|_| self.move_box(from_stack, to_stack));
            },
            Model::Model9001 => {
                self.move_boxes_at_once(number_to_move, from_stack, to_stack)
            }
        }
    }

    pub fn move_box(&mut self, from_stack: usize, to_stack: usize) {
        let b = self.stacks[from_stack].pop().expect("Tried to move a box from a stack with no boxes.");
        self.stacks[to_stack].push(b);
    }

    pub fn move_boxes_at_once(&mut self, number_to_move: usize, from_stack: usize, to_stack: usize) {
        let boxes = (0..number_to_move).map(
            |_| self.stacks[from_stack].pop().expect("Tried to move from a stack with no boxes.")
        ).collect::<Vec<_>>();
        let boxes = boxes.into_iter().rev().collect::<Vec<_>>();
        boxes.into_iter().for_each(|b| self.stacks[to_stack].push(b));
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Instruction {
    number_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Instruction {

    pub fn new(number_to_move: usize, from_stack: usize, to_stack: usize) -> Self {
        Self {
            number_to_move,
            from_stack,
            to_stack,
        }
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let words = s.split_whitespace().collect::<Vec<_>>();
        assert_eq!(words[0], "move");
        let number_to_move = words[1].parse::<usize>().map_err(|_| "Couldn't parse number to move.".to_string())?;
        assert_eq!(words[2], "from");
        let from_stack = words[3].parse::<usize>().map_err(|_| "Couldn't parse from".to_string()).map(|n| n - 1)?;
        assert_eq!(words[4], "to");
        let to_stack = words[5].parse::<usize>().map_err(|_| "Couldn't parse to.".to_string()).map(|n| n - 1)?;
        Ok(Self::new(number_to_move, from_stack, to_stack))
    }

    pub fn unpack(&self) -> (usize, usize, usize) {
        (self.number_to_move, self.from_stack, self.to_stack)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Model {
    Model9000,
    Model9001,
}

#[cfg(test)]
mod test_problem_05 {

    use super::*;

    #[test]
    fn test_problem_05a_passes() {
        
        let input = InputParser::new().parse_to_single_string("input_05.txt").unwrap();
        
        let example = 
            "    [D]    \n".to_string() + 
            &"[N] [C]    \n".to_string() + 
            &"[Z] [M] [P]\n".to_string() + 
            &" 1   2   3 \n".to_string() + 
            &"\n".to_string() + 
            &"move 1 from 2 to 1\n".to_string() + 
            &"move 3 from 1 to 3\n".to_string() + 
            &"move 2 from 2 to 1\n".to_string() + 
            &"move 1 from 1 to 2".to_string(); 
        
        assert_eq!(solve_problem_05a(example), "CMZ".to_string());

        let answer = solve_problem_05a(input);
        assert_eq!(answer, "BZLVHBWQF".to_string());
    }
    
    #[test]
    fn test_problem_05b_passes() {
        let input = InputParser::new().parse_to_single_string("input_05.txt").unwrap();
        
        let example = 
            "    [D]    \n".to_string() + 
            &"[N] [C]    \n".to_string() + 
            &"[Z] [M] [P]\n".to_string() + 
            &" 1   2   3 \n".to_string() + 
            &"\n".to_string() + 
            &"move 1 from 2 to 1\n".to_string() + 
            &"move 3 from 1 to 3\n".to_string() + 
            &"move 2 from 2 to 1\n".to_string() + 
            &"move 1 from 1 to 2".to_string(); 
        
        assert_eq!(solve_problem_05b(example), "MCD".to_string());

        let answer = solve_problem_05b(input);
        assert_eq!(answer, "TDGJQTZSL".to_string());
    }

    #[test]
    fn test_stacks_can_move_boxes() {
        let mut stacks = Stacks::new(
            vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P'],
            ]
        );

        let instructions = vec![
            Instruction::new(1, 1, 0),
            Instruction::new(3, 0, 2),
            Instruction::new(2, 1, 0),
            Instruction::new(1, 0, 1),
        ];

        stacks.handle_instructions(instructions, Model::Model9000);

        assert_eq!(stacks.get_top_boxes(), vec!['C', 'M', 'Z']);
        
    }

    #[test]
    fn test_stacks_can_move_box() {

        let mut stacks = Stacks::new(
            vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P'],
            ]
        );

        stacks.handle_instruction(Instruction::new(1, 2, 1), Model::Model9000);

        assert_eq!(stacks.get_stack(1), &vec!['M', 'C', 'D', 'P']);
        assert_eq!(stacks.get_stack(0).len(), 2);
        assert_eq!(stacks.get_stack(2).len(), 0);
        assert_eq!(stacks.get_n_boxes(), 6);   
        
    }

    #[test]
    fn test_stacks_parses_from_string() {

        let s = 
           "    [D]    \n".to_string() +
          &"[N] [C]    \n".to_string() +
          &"[Z] [M] [P]".to_string();
        
        let stacks = Stacks::from_string(s).unwrap();

        assert_eq!(stacks.get_n_boxes(), 6);
        assert_eq!(stacks.get_stack(0), &vec!['Z', 'N']);
        assert_eq!(stacks.get_stack(1), &vec!['M', 'C', 'D']);
    }

    #[test]
    fn test_parse_row() {
        
        let s = "[Z] [M] [P]".to_string();
        assert_eq!(Stacks::parse_row(s).unwrap(), vec![Some('Z'), Some('M'), Some('P')]);
        
        let s = "[N] [C]    ".to_string();

        assert_eq!(Stacks::parse_row(s).unwrap(), vec![Some('N'), Some('C'), None]);
    }

    #[test]
    fn test_instruction_parses_from_string() {
        
        assert_eq!(Instruction::from_string("move 3 from 1 to 3".to_string()), Ok(Instruction::new(3, 0, 2)));

    }

}