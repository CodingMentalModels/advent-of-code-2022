use crate::input::input::InputParser;

pub fn solve_problem_12a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_12b(input: Vec<String>) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test_problem_12 {

    use super::*;

    #[test]
    fn test_problem_12a_passes() {
        
        let input = InputParser::new().parse_as_string("input_12.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_12a(shorted_input), 0);

        let answer = solve_problem_12a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_12b_passes() {
        let input = InputParser::new().parse_as_string("input_12.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_12b(shorted_input), 0);

        let answer = solve_problem_12b(input);
        assert_eq!(answer, 0);
    }

}