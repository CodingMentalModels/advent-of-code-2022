use std::collections::HashSet;

use crate::input::input::InputParser;

const AMOUNT_TO_SUBTRACT_FROM_LOWERCASE: u32 = 96;
const AMOUNT_TO_SUBTRACT_FROM_UPPERCASE: u32 = 38;

fn solve_problem_03a(input: Vec<String>) -> u32 {
    input.into_iter().map(|rucksack| get_priority(get_common_element(rucksack))).sum()
}

fn solve_problem_03b(input: Vec<String>) -> u32 {
    let teams = InputParser::chunk(input, 3).expect("We should be able to chunk the input into thirds.");
    teams.into_iter().map(|team| get_priority(get_common_element_from_vec(team))).sum()
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - AMOUNT_TO_SUBTRACT_FROM_LOWERCASE
    } else if c.is_uppercase() {
        c as u32 - AMOUNT_TO_SUBTRACT_FROM_UPPERCASE
    } else {
        panic!("Input was neither lower nor uppercase: {}", c);
    }
}

fn get_common_element(rucksack: String) -> char {
    let length = rucksack.len();
    assert_eq!(length % 2, 0);
    let half_length = length / 2;
    let rucksack_chars = rucksack.chars();
    let first_half = rucksack_chars.clone().take(half_length).collect::<HashSet<_>>();
    let second_half = rucksack_chars.skip(half_length).take(half_length).collect::<HashSet<_>>();
    let intersection: Vec<_> = first_half.intersection(&second_half).collect();
    assert_eq!(intersection.len(), 1);
    
    *intersection[0]
}

fn get_common_element_from_vec(strings: Vec<String>) -> char {
    let intersection = strings.into_iter().map(|s| s.chars().collect::<HashSet<_>>())
        .reduce(
            |accumulator, element| accumulator.intersection(&element).map(
                |c| *c)
            .collect::<HashSet<char>>()
        ).expect("At least one element should be in the starting vector.");
    assert_eq!(intersection.len(), 1);
    return intersection.into_iter().next().unwrap();
}

#[cfg(test)]
mod test_problem_03 {

    use super::*;

    #[test]
    fn test_problem_03a_passes() {
        
        let input = InputParser::new().parse_as_string("input_03.txt").unwrap();
        
        let example = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(solve_problem_03a(example), 157);
        
        let shortened_input = input.iter().take(3).map(|i| i.clone()).collect();

        // rNZNWvMZZmDDmwqNdZrWTqhJMhhgzggBhzBJBchQzzJJ => M => 39
        // pHlSVbVbFHgHBzzhQHqg => H => 26 + 8 = 34
        // nVsqGpbbtDtTNmrmfZ => t => 20

        // 39 + 34 + 20 = 93
        assert_eq!(solve_problem_03a(shortened_input), 93);

        let answer = solve_problem_03a(input);
        assert_eq!(answer, 7691);
    }
    
    #[test]
    fn test_problem_03b_passes() {
        let input = InputParser::new().parse_as_string("input_03.txt").unwrap();
        
        let example = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        
        assert_eq!(solve_problem_03b(example), 70);

        let answer = solve_problem_03b(input);
        assert_eq!(answer, 2508);
    }

    #[test]
    fn test_gets_common_element() {
        
        assert_eq!(get_common_element("rNZNWvMZZmDDmwqNdZrWTqhJMhhgzggBhzBJBchQzzJJ".to_string()), 'M');
        assert_eq!(get_common_element("pHlSVbVbFHgHBzzhQHqg".to_string()), 'H');
        assert_eq!(get_common_element("nVsqGpbbtDtTNmrmfZ".to_string()), 't');

    }

    #[test]
    fn test_gets_common_element_from_vec() {

        let v = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];

        assert_eq!(get_common_element_from_vec(v), 'r');


        let v = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];

        assert_eq!(get_common_element_from_vec(v), 'Z');

    }

    #[test]
    fn test_letters_to_priority() {

        assert_eq!('a' as u32 - AMOUNT_TO_SUBTRACT_FROM_LOWERCASE, 1);
        assert_eq!('Z' as u32 - AMOUNT_TO_SUBTRACT_FROM_UPPERCASE, 52);

        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('b'), 2);
        assert_eq!(get_priority('q'), 17);
        assert_eq!(get_priority('H'), 34);
        assert_eq!(get_priority('t'), 20);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_char_to_u32() {
        assert_eq!('a' as u32, 97);
        assert_eq!('b' as u32, 98);
        assert_eq!('c' as u32, 99);
        assert_eq!('z' as u32, 122);
        assert_eq!('A' as u32, 65);
        assert_eq!('B' as u32, 66);
        assert_eq!('C' as u32, 67);
        assert_eq!('Z' as u32, 90);
    }
}