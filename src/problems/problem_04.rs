use crate::input::input::InputParser;

type Range = (u32, u32);
type Pair = (Range, Range);

fn solve_problem_04a(input: Vec<String>) -> u32 {
    input.into_iter()
        .map(|s| parse_cleaning_pair(s))
        .filter(|p| has_completely_overlapping(*p))
        .count() as u32
}

fn solve_problem_04b(input: Vec<String>) -> u32 {
    input.into_iter()
        .map(|s| parse_cleaning_pair(s))
        .filter(|p| overlaps(*p))
        .count() as u32
}

fn parse_cleaning_pair(s: String) -> Pair {
    let ranges = s.split(",").map(|s| parse_range(s)).collect::<Vec<_>>();
    assert_eq!(ranges.len(), 2);
    return (ranges[0], ranges[1]);
}

fn parse_range(s: &str) -> (u32, u32) {
    let pair = s.split("-")
        .map(
            |n| n.parse::<u32>()
            .expect(&format!("Should be parsable into a u32: {}", n))
        ).collect::<Vec<_>>();
    assert_eq!(pair.len(), 2);
    (pair[0], pair[1])
}

fn overlaps(pair: Pair) -> bool {
    !are_sequential(pair)
}

fn are_sequential(pair: Pair) -> bool {
    are_sequential_in_order(pair) || are_sequential_in_inverse_order(pair)
}

fn are_sequential_in_order(pair: Pair) -> bool {
    let (left, right) = pair;
    left.1 < right.0
}

fn are_sequential_in_inverse_order(pair: Pair) -> bool {
    are_sequential_in_order((pair.1, pair.0))
}

fn has_completely_overlapping(pair: Pair) -> bool {
    is_left_subset_of_right(pair) || is_right_subset_of_left(pair)
}

fn is_left_subset_of_right(pair: Pair) -> bool {
    let left = pair.0;
    let right = pair.1;
    (left.0 >= right.0) && (left.1 <= right.1)
}

fn is_right_subset_of_left(pair: Pair) -> bool {
    is_left_subset_of_right((pair.1, pair.0))
}

#[cfg(test)]
mod test_problem_04 {

    use super::*;

    #[test]
    fn test_problem_04a_passes() {
        
        let input = InputParser::new().parse_as_string("input_04.txt").unwrap();
        
        let example = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),

        ];

        assert_eq!(solve_problem_04a(example), 2);

        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_04a(shorted_input), 6);

        let answer = solve_problem_04a(input);
        assert_eq!(answer, 542);
    }
    
    #[test]
    fn test_problem_04b_passes() {
        let input = InputParser::new().parse_as_string("input_04.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();
        
        let example = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),

        ];

        assert_eq!(solve_problem_04b(example), 4);


        assert_eq!(solve_problem_04b(shorted_input), 8);

        let answer = solve_problem_04b(input);
        assert_eq!(answer, 900);
    }

}