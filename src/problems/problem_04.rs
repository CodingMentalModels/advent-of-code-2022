use crate::input::input::InputParser;

pub fn solve_problem_04a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_04b(input: Vec<String>) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test_problem_04 {

    use super::*;

    #[test]
    fn test_problem_04a_passes() {
        
        let input = InputParser::new().parse_as_string("input_04.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_04a(shorted_input), 0);

        let answer = solve_problem_04a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_04b_passes() {
        let input = InputParser::new().parse_as_string("input_04.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_04b(shorted_input), 0);

        let answer = solve_problem_04b(input);
        assert_eq!(answer, 0);
    }

}