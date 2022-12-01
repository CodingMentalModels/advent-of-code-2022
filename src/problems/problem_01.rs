use crate::input::input::InputParser;

pub fn solve_problem_01a(input: Vec<Vec<String>>) -> usize {
    input.into_iter().map(
        |chunk| chunk.into_iter().map(|s| s.parse::<usize>().expect("All strings should be valid numbers.")).sum()
    ).max().expect("The iterator shouldn't be empty.")
}

fn solve_problem_01b(input: Vec<Vec<String>>) -> usize {
    let mut calorie_sums: Vec<usize> = input.into_iter().map(
        |chunk| chunk.into_iter().map(|s| s.parse::<usize>().expect("All strings should be valid numbers.")).sum()
    ).collect::<Vec<_>>();
    calorie_sums.sort();
    calorie_sums.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod test_problem_01 {

    use super::*;

    #[test]
    fn test_problem_01a_passes() {
        
        let input = InputParser::new().parse_as_string_chunks("input_01.txt", "\n\n").unwrap();
        let shorted_input = input.iter().take(2).map(|i| i.clone()).collect();

        assert_eq!(shorted_input, 
            vec![
                vec![
                    "2832".to_string(),
                    "2108".to_string(),
                    "3082".to_string(),
                    "4328".to_string(),
                    "6843".to_string(),
                    "5121".to_string(),
                    "2869".to_string(),
                    "1366".to_string(),
                    "2358".to_string(),
                    "1680".to_string(),
                    "4980".to_string(),
                    "1161".to_string(),
                ],
                vec![
                    "8026".to_string(),
                    "2154".to_string(),
                    "4242".to_string(),
                    "1023".to_string(),
                    "2744".to_string(),
                    "3162".to_string(),
                    "4093".to_string(),
                    "1150".to_string(),
                    "5397".to_string(),
                    "2738".to_string(),
                    "5657".to_string(),
                ]
            ]
        );
        assert_eq!(solve_problem_01a(shorted_input), 40386);

        let answer = solve_problem_01a(input);
        assert_eq!(answer, 67622);
    }
    
    #[test]
    fn test_problem_01b_passes() {
        let input = InputParser::new().parse_as_string_chunks("input_01.txt", "\n\n").unwrap();

        let shorted_input = input.iter().take(4).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_01b(shorted_input), 147861);

        let answer = solve_problem_01b(input);
        assert_eq!(answer, 201491);
    }

}