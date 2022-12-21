use crate::{input::input::InputParser, utility::vector::Vec2};

pub fn solve_problem_12a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_12b(input: Vec<String>) -> usize {
    unimplemented!();
}

fn to_height(c: char) -> usize {
    let to_return = (c as usize) - ('a' as usize);
    assert!(to_return <= 25);
    return to_return;
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Grid {
    starting_point: Vec2,
    ending_point: Vec2,
    grid: Vec<Vec<usize>>,
}

impl Grid {

    pub fn new(starting_point: Vec2, ending_point: Vec2, grid: Vec<Vec<usize>>) -> Self {
        Self { starting_point, ending_point, grid }
    }
    
    pub fn from_strings(strings: Vec<String>) -> Self {
        let (starting_point, ending_point) = Self::parse_starting_and_ending_points(&strings);
        let grid = strings.into_iter().map(|s| Self::parse_row(s)).collect();
        Self::new(starting_point, ending_point, grid)
    }

    fn parse_starting_and_ending_points(strings: &Vec<String>) -> (Vec2, Vec2) {
        let maybe_points = strings.iter().enumerate()
            .map(
                |(i, s)| (s.find('S').map(|j| (i, j)), s.find('E').map(|j| (i, j)))
            ).reduce(|accumulator, element| {
                match element {
                    (Some(x), Some(y)) => (Some(x), Some(y)),
                    (Some(x), _) => (Some(x), accumulator.1),
                    (_, Some(y)) => (accumulator.0, Some(y)),
                    _ => accumulator
                }
            }
        ).expect("There should always be at least one element in the strings.");
        let starting_point = maybe_points.0.expect("maybe_points.0 was still None");
        let ending_point = maybe_points.1.expect("maybe_points.1 was still None");
        return (Vec2::new(starting_point.0 as i32, starting_point.1 as i32), Vec2::new(ending_point.0 as i32, ending_point.1 as i32));
    }

    fn parse_row(s: String) -> Vec<usize> {
        s.chars().map(|c| Self::parse_char(c)).collect()
    }

    fn parse_char(c: char) -> usize {
        match c {
            'S' => 0,
            'E' => to_height('z'),
            c => to_height(c)
        }
    }

    pub fn get(&self, v: Vec2) -> usize {
        assert!(v.x() >= 0);
        assert!(v.y() >= 0);
        self.grid[v.x() as usize][v.y() as usize]
    }

    pub fn get_starting_point(&self) -> Vec2 {
        self.starting_point
    }

    pub fn get_ending_point(&self) -> Vec2 {
        self.ending_point
    }

    pub fn get_shortest_path(&self, start: Vec2, end: Vec2) -> Option<Vec<Vec2>> {
        unimplemented!()
    }
}


#[cfg(test)]
mod test_problem_12 {

    use crate::utility::vector::Vec2;

    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "Sabqponm".to_string(),
            "abcryxxl".to_string(),
            "accszExk".to_string(),
            "acctuvwj".to_string(),
            "abdefghi".to_string(),
        ]
    }

    #[test]
    fn test_problem_12a_passes() {
        
        assert_eq!(solve_problem_12a(get_example_input()), 31);

        let input = InputParser::new().parse_as_string("input_12.txt").unwrap();

        let answer = solve_problem_12a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_12b_passes() {
        let input = InputParser::new().parse_as_string("input_12.txt").unwrap();

        let answer = solve_problem_12b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_gets_shortest_path() {
        
        let grid = Grid::from_strings(get_example_input());

        assert_eq!(grid.get_shortest_path(Vec2::new(0, 0), Vec2::new(0, 0)), Some(vec![Vec2::new(0, 0)]));

        assert_eq!(grid.get_shortest_path(Vec2::new(0, 0), Vec2::new(0, 1)), Some(vec![Vec2::new(0, 0), Vec2::new(0, 1)]));
        assert_eq!(grid.get_shortest_path(Vec2::new(0, 0), Vec2::new(1, 0)), Some(vec![Vec2::new(0, 0), Vec2::new(1, 0)]));
        assert_eq!(grid.get_shortest_path(Vec2::new(1, 7), Vec2::new(0, 7)), Some(vec![Vec2::new(1, 7), Vec2::new(0, 7)]));

        assert_eq!(grid.get_shortest_path(Vec2::new(0, 0), Vec2::new(1, 1)), Some(
            vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(1, 1),
                ]
            )
        );

        assert_eq!(grid.get_shortest_path(grid.get_starting_point(), grid.get_ending_point()), Some(
            vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(1, 1),
                Vec2::new(2, 1),
                Vec2::new(2, 2),
                Vec2::new(3, 2),
                Vec2::new(4, 2),
                Vec2::new(4, 3),
                Vec2::new(4, 4),
                Vec2::new(4, 5),
                Vec2::new(4, 6),
                Vec2::new(4, 7),
                Vec2::new(3, 7),
                Vec2::new(2, 7),
                Vec2::new(1, 7),
                Vec2::new(0, 7),
                Vec2::new(0, 6),
                Vec2::new(0, 5),
                Vec2::new(0, 4),
                Vec2::new(0, 3),
                Vec2::new(1, 3),
                Vec2::new(2, 3),
                Vec2::new(3, 3),
                Vec2::new(3, 4),
                Vec2::new(3, 5),
                Vec2::new(3, 6),
                Vec2::new(2, 6),
                Vec2::new(1, 6),
                Vec2::new(1, 5),
                Vec2::new(1, 4),
                Vec2::new(2, 4),
                Vec2::new(2, 5),
                ]
            )
        );

    }

    #[test]
    fn test_parses_input() {
        
        let grid = Grid::from_strings(get_example_input());

        assert_eq!(grid.get_starting_point(), Vec2::new(0, 0));
        assert_eq!(grid.get(Vec2::new(0, 0)), to_height('a'));

        assert_eq!(grid.get_ending_point(), Vec2::new(2, 5));
        assert_eq!(grid.get(Vec2::new(2, 5)), to_height('z'));

        assert_eq!(grid.get(Vec2::new(2, 1)), to_height('c'));
        assert_eq!(grid.get(Vec2::new(1, 2)), to_height('c'));
        assert_eq!(grid.get(Vec2::new(4, 7)), to_height('i'));

    }

    #[test]
    fn test_to_height() {
        assert_eq!(to_height('a'), 0);
        assert_eq!(to_height('b'), 1);
        assert_eq!(to_height('m'), 12);
        assert_eq!(to_height('z'), 25);
    }
}