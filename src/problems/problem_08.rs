use crate::input::input::InputParser;

pub fn solve_problem_08a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_08b(input: Vec<String>) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test_problem_08 {

    use super::*;

    #[test]
    fn test_problem_08a_passes() {
        
        let input = InputParser::new().parse_as_string("input_08.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_08a(shorted_input), 0);

        let answer = solve_problem_08a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_08b_passes() {
        let input = InputParser::new().parse_as_string("input_08.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_08b(shorted_input), 0);

        let answer = solve_problem_08b(input);
        assert_eq!(answer, 0);
    }

}