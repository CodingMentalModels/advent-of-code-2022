use crate::input::input::InputParser;

pub fn solve_problem_02a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_02b(input: Vec<String>) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test_problem_02 {

    use super::*;

    #[test]
    fn test_problem_02a_passes() {
        
        let input = InputParser::new().parse_as_string("input_02.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_02a(shorted_input), 0);

        let answer = solve_problem_02a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_02b_passes() {
        let input = InputParser::new().parse_as_string("input_02.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_02b(shorted_input), 0);

        let answer = solve_problem_02b(input);
        assert_eq!(answer, 0);
    }

}