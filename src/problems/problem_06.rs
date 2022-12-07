use std::collections::HashSet;

use crate::input::input::InputParser;

fn solve_problem_06a(input: String) -> usize {
    let mut subroutine = Subroutine::new();
    for (i, c) in input.chars().enumerate() {
        subroutine.consume(c);
        if subroutine.last_n_were_different(4) {
            return i + 1;
        }
    }
    panic!("Got to the end of the string without finding a distinct group.");
}

fn solve_problem_06b(input: String) -> usize {
    let mut subroutine = Subroutine::new();
    for (i, c) in input.chars().enumerate() {
        subroutine.consume(c);
        if subroutine.last_n_were_different(14) {
            return i + 1;
        }
    }
    panic!("Got to the end of the string without finding a distinct group.");
}

struct Subroutine {
    consumed: Vec<char>,
}

impl Subroutine {

    pub fn new() -> Self {
        Self {consumed: Vec::new()}
    }

    pub fn consume(&mut self, c: char) {
        self.consumed.push(c);
    }

    pub fn len(&self) -> usize {
        self.consumed.len()
    }

    pub fn last_n_were_different(&self, n: usize) -> bool {
        if (n > self.len()) {
            return false;
        }
        (0..n).into_iter()
            .map(|i| self.consumed[self.consumed.len() - i - 1])
            .collect::<HashSet<_>>().len() == n
    }
}

#[cfg(test)]
mod test_problem_06 {

    use super::*;

    #[test]
    fn test_problem_06a_passes() {

        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(solve_problem_06a(example), 7);

        let example = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(solve_problem_06a(example), 5);

        let example = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(solve_problem_06a(example), 6);

        let example = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(solve_problem_06a(example), 10);

        let input = InputParser::new().parse_to_single_string("input_06.txt").unwrap();

        let answer = solve_problem_06a(input);
        assert_eq!(answer, 1647);
    }
    
    #[test]
    fn test_problem_06b_passes() {
        let example = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(solve_problem_06b(example), 19);

        let example = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(solve_problem_06b(example), 23);

        let example = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(solve_problem_06b(example), 23);

        let example = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(solve_problem_06b(example), 29);

        let example = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(solve_problem_06b(example), 26);

        let input = InputParser::new().parse_to_single_string("input_06.txt").unwrap();

        let answer = solve_problem_06b(input);
        assert_eq!(answer, 2447);
    }

    #[test]
    fn test_subroutine_compares_last_n() {
        
        let mut subroutine = Subroutine::new();
        subroutine.consume('A');
        assert!(subroutine.last_n_were_different(1));
        assert!(!subroutine.last_n_were_different(4));
        
        subroutine.consume('B');
        subroutine.consume('C');

        assert!(subroutine.last_n_were_different(2));
        assert!(subroutine.last_n_were_different(3));
        assert!(!subroutine.last_n_were_different(4));

        subroutine.consume('B');

        assert!(subroutine.last_n_were_different(2));
        assert!(!subroutine.last_n_were_different(3));
        assert!(!subroutine.last_n_were_different(4));
    }

}