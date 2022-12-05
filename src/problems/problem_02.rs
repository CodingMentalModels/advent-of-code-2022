use crate::input::input::InputParser;

fn solve_problem_02a(input: Vec<(String, String)>) -> u32 {
    input.into_iter().map(|(lhs, rhs)| HandShape::score_round(
        &HandShape::from_string_problem_02a(lhs),
        &HandShape::from_string_problem_02a(rhs)
    )).sum()
}

fn solve_problem_02b(input: Vec<(String, String)>) -> u32 {
    input.into_iter().map(|(lhs, rhs)| HandShape::score_round(
        &Outcome::from_string(&lhs).get_hand_shape_to_achieve(&HandShape::from_string(rhs.clone())),
        &HandShape::from_string(rhs)
    )).sum()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum HandShape {
    Rock,
    Paper,
    Scissors
}

impl HandShape {

    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Invalid hand shape: {}", s)
        }
    }

    pub fn from_string_problem_02a(s: String) -> Self {
        match s.as_str() {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid hand shape: {}", s)
        }
    }

    pub fn to_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn evaluate(lhs: &HandShape, rhs: &HandShape) -> Outcome {
        if lhs == rhs { return Outcome::Draw; };

        match (lhs, rhs) {
            (Self::Rock, Self::Paper) => { Outcome::Lose },
            (Self::Rock, Self::Scissors) => { Outcome::Win },
            (Self::Paper, Self::Rock) => { Outcome::Win },
            (Self::Paper, Self::Scissors) => { Outcome::Lose },
            (Self::Scissors, Self::Rock) => { Outcome::Lose },
            (Self::Scissors, Self::Paper) => { Outcome::Win },
            _ => { panic!("It's a draw, but that shouldn't be possible.")}
        }
    }

    pub fn score_round(lhs: &HandShape, rhs: &HandShape) -> u32 {
        Self::evaluate(lhs, rhs).score() + lhs.to_score()
    }

}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {

    pub fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid outcome! {}", s)
        }
    }

    pub fn get_hand_shape_to_achieve(&self, opponent_shape: &HandShape) -> HandShape {
        if self == &Self::Draw { return opponent_shape.clone() };
        match (opponent_shape, self) {
            (&HandShape::Rock, Self::Win) => { HandShape::Paper },
            (&HandShape::Rock, Self::Lose) => { HandShape::Scissors },
            (&HandShape::Paper, Self::Win) => { HandShape::Scissors },
            (&HandShape::Paper, Self::Lose) => { HandShape::Rock },
            (&HandShape::Scissors, Self::Win) => { HandShape::Rock }, 
            (&HandShape::Scissors, Self::Lose) => { HandShape::Paper }, 
            (_, Self::Draw) => panic!("We needed to draw but didn't return above.")
        }
    }

}

#[cfg(test)]
mod test_problem_02 {

    use super::*;

    #[test]
    fn test_problem_02a_passes() {
        
        let input: Vec<_> = InputParser::new().parse_as_string("input_02.txt").unwrap()
            .into_iter().map(|x| x.chars().collect::<Vec<_>>()).map(|x| (x[2].to_string(), x[0].to_string())).collect();
        
        assert_eq!(input.len(), 2500);

        let example_input = vec![
            ("A".to_string(), "Y".to_string()),
            ("B".to_string(), "X".to_string()),
            ("C".to_string(), "Z".to_string()),
        ];

        assert_eq!(solve_problem_02a(example_input), 15);
        // A Y => Y A => Paper vs. Rock => 6 + 2 = 8
        // B Z => Z B => Scissors vs. Paper => 6 + 3 = 9
        // C Y => Y C => Paper vs. Scissors => 0 + 2 = 2
        // B Y => Y B => Paper vs. Paper => 3 + 2 = 5
        // A Y => Y A => Paper vs. Rock => 6 + 2 = 8

        // => 32

        let shorted_input = input.iter().take(5).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_02a(shorted_input), 32);

        let answer = solve_problem_02a(input);

        assert_eq!(answer, 15632);
    }
    
    #[test]
    fn test_problem_02b_passes() {
        
        let input: Vec<_> = InputParser::new().parse_as_string("input_02.txt").unwrap()
            .into_iter().map(|x| x.chars().collect::<Vec<_>>()).map(|x| (x[2].to_string(), x[0].to_string())).collect();
        
        // A Y => Y A => Draw vs. Rock => 3 + 1 = 4
        // B Z => Z B => Win vs. Paper => 6 + 3 = 9
        // C Y => Y C => Draw vs. Scissors => 3 + 3 = 6
        // B Y => Y B => Draw vs. Paper => 3 + 2 = 5
        // A Y => Y A => Draw vs. Rock => 3 + 1 = 4

        // => 28

        let shorted_input = input.iter().take(5).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_02b(shorted_input), 28);

        let answer = solve_problem_02b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_can_compute_round_score() {
        assert_eq!(HandShape::score_round(&HandShape::Rock, &HandShape::Rock), 4);
        assert_eq!(HandShape::score_round(&HandShape::Scissors, &HandShape::Paper), 9);
        assert_eq!(HandShape::score_round(&HandShape::Paper, &HandShape::Scissors), 2);
    }
}