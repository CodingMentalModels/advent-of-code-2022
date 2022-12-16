use crate::input::input::InputParser;

pub fn solve_problem_10a(input: Vec<String>) -> i32 {
    let instructions = input.into_iter().map(|s| Instruction::from_string(&s)).collect();
    let mut cpu = Cpu::new();
    cpu.consume_all(instructions);
    cpu.get_signal_strength()
}

fn solve_problem_10b(input: Vec<String>) -> Vec<String> {
        let mut crt = Crt::new();

        let mut cpu = Cpu::new();
        let instructions = input.into_iter().map(|s| Instruction::from_string(&s)).collect();
        cpu.consume_all(instructions);

        crt.render(cpu)
}

#[derive(Clone, Debug)]
struct Crt {
    pixels: Vec<String>,
}

impl Crt {

    pub fn new() -> Self {
        Self {
            pixels: vec![
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ]
        }
    }

    pub fn render(&mut self, cpu: Cpu) -> Vec<String> {
        cpu.register_history.into_iter().skip(1).enumerate()
            .for_each(|(i, x)| {
                self.push((x - (i % 40) as i32).abs() <= 1)
            }
        );
        return self.pixels.clone();
    }

    fn push(&mut self, is_lit: bool) {
        let idx = self.get_next_to_add();
        self.pixels[idx].push(Self::get_symbol(is_lit));
    }

    fn get_next_to_add(&self) -> usize {
        for (i, row) in self.pixels.iter().enumerate() {
            assert!(row.len() <= 40);
            if row.len() != 40 {
                return i;
            }
        }
        panic!();
    }

    fn get_symbol(is_lit: bool) -> char {
        if is_lit {
            '#'
        } else {
            '.'
        }
    }
}
    

#[derive(Clone, Debug)]
struct Cpu {
    register: i32,
    register_history: Vec<i32>,
}

impl Cpu {

    pub fn new() -> Self {
        Self {
            register: 1,
            register_history: vec![1],
        }
    }

    pub fn get_cycles(&self) -> usize {
        self.register_history.len() - 1
    }

    pub fn get_register(&self) -> i32 {
        self.register
    }

    pub fn get_historical_register(&self, i: usize) -> i32 {
        self.register_history[i]
    }

    pub fn get_signal_strength(&self) -> i32 {
        assert!(self.register_history.len() >= 220);
        20*self.register_history[20] +
            60*self.register_history[60] +
            100*self.register_history[100] +
            140*self.register_history[140] +
            180*self.register_history[180] +
            220*self.register_history[220]
    }

    pub fn consume_all(&mut self, instructions: Vec<Instruction>) {
        instructions.into_iter().for_each(|i| self.consume(i));
    }

    pub fn consume(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NoOp => {
                self.register_history.push(self.register);
            },
            Instruction::AddX(x) => {
                self.register_history.push(self.register);
                self.register_history.push(self.register);
                self.register += x;
            }
        }
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {

    pub fn from_string(s: &str) -> Self {
        let words: Vec<_> = s.split_whitespace().collect();
        assert!(words.len() > 0);
        assert!(words.len() <= 2);

        if words.len() == 1 {
            assert_eq!(words[0], "noop");
            return Self::NoOp;
        }

        assert_eq!(words[0], "addx");
        let x = words[1].parse::<i32>().expect("X should be parsable as an i32");
        return Self::AddX(x);
    }
}

#[cfg(test)]
mod test_problem_10 {

    use std::iter;

    use super::*;

    #[test]
    fn test_problem_10a_passes() {
        
        let example_input = InputParser::new().parse_as_string("example_input_10.txt").unwrap();
        assert_eq!(solve_problem_10a(example_input), 13140);

        let input = InputParser::new().parse_as_string("input_10.txt").unwrap();

        let answer = solve_problem_10a(input);
        assert_eq!(answer, 14040);
    }
    
    #[test]
    fn test_problem_10b_passes() {
        let example_input = InputParser::new().parse_as_string("example_input_10.txt").unwrap();
        assert_eq!(solve_problem_10b(example_input), get_problem_10b_example_output());

        let input = InputParser::new().parse_as_string("input_10.txt").unwrap();

        let answer = solve_problem_10b(input);
        let expected = vec![
            "####..##...##....##.####...##.####.#....".to_string(),
            "...#.#..#.#..#....#....#....#.#....#....".to_string(),
            "..#..#....#.......#...#.....#.###..#....".to_string(),
            ".#...#.##.#.......#..#......#.#....#....".to_string(),
            "#....#..#.#..#.#..#.#....#..#.#....#....".to_string(),
            "####..###..##...##..####..##..#....####.".to_string(),
        ];
        assert_eq!(answer, expected);
    }

    fn get_problem_10b_example_output() -> Vec<String> {
        vec![
            "##..##..##..##..##..##..##..##..##..##..".to_string(),
            "###...###...###...###...###...###...###.".to_string(),
            "####....####....####....####....####....".to_string(),
            "#####.....#####.....#####.....#####.....".to_string(),
            "######......######......######......####".to_string(),
            "#######.......#######.......#######.....".to_string(),
        ]
    }
    
    #[test]
    fn test_cpu_gets_signal_strength() {
        
        let mut cpu = Cpu::new();
        let instructions = iter::repeat(Instruction::NoOp).take(240).collect();
        cpu.consume_all(instructions);
        assert_eq!(cpu.get_signal_strength(), 20 + 60 + 100 + 140 + 180 + 220);

        let mut cpu = Cpu::new();
        let instructions = iter::repeat(Instruction::AddX(1)).take(240).collect();
        cpu.consume_all(instructions);
        assert_eq!(cpu.get_signal_strength(), 20*10 + 60*30 + 100*50 + 140*70 + 180*90 + 220*110);
        
        let mut cpu = Cpu::new();
        let instructions = vec![
            Instruction::NoOp,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ];
        cpu.consume_all(instructions);
        assert_eq!(
            cpu.register_history,
            vec![
                1,
                1,
                1,
                1,
                4,
                4,
            ]
        );

        let mut cpu = Cpu::new();
        let example_input = InputParser::new().parse_as_string("example_input_10.txt").unwrap();
        let instructions = example_input.into_iter().map(|s| Instruction::from_string(&s)).collect();
        cpu.consume_all(instructions);

        assert_eq!(
            cpu.clone().register_history.into_iter().take(21).collect::<Vec<i32>>(),
            vec![
                1,
                1,
                1,
                16,
                16,
                5,
                5,
                11,
                11,
                8,
                8,
                13,
                13,
                12,
                12,
                4,
                4,
                17,
                17,
                21,
                21,
            ]
        );
        assert_eq!(cpu.get_historical_register(20), 21);
        assert_eq!(cpu.get_historical_register(60), 19);
        assert_eq!(cpu.get_historical_register(100), 18);
        assert_eq!(cpu.get_historical_register(140), 21);
        assert_eq!(cpu.get_historical_register(180), 16);
        assert_eq!(cpu.get_historical_register(220), 18);
        

    }

    #[test]
    fn test_cpu_consumes_instructions() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.get_cycles(), 0);
        assert_eq!(cpu.get_register(), 1);

        cpu.consume(Instruction::NoOp);
        assert_eq!(cpu.get_cycles(), 1);
        assert_eq!(cpu.get_register(), 1);

        cpu.consume(Instruction::AddX(-5));
        assert_eq!(cpu.get_cycles(), 3);
        assert_eq!(cpu.get_register(), -4);
    }

    #[test]
    fn test_crt_render() {
        
        let mut crt = Crt::new();

        let mut cpu = Cpu::new();
        let example_input = InputParser::new().parse_as_string("example_input_10.txt").unwrap();
        let instructions = example_input.into_iter().map(|s| Instruction::from_string(&s)).collect();
        cpu.consume_all(instructions);

        assert_eq![
            crt.render(cpu),
            get_problem_10b_example_output()
        ];

    }
}