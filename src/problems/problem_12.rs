use std::{collections::{HashSet, HashMap}, iter};

use crate::{input::input::InputParser, utility::vector::Vec2};

pub fn solve_problem_12a(input: Vec<String>) -> usize {
    let grid = Grid::from_strings(input);
    let shortest_path = grid.get_shortest_path_between(
        false,
        grid.get_starting_point(),
        grid.get_ending_point()
    );
    return shortest_path.unwrap().len() - 1;
}

fn solve_problem_12b(input: Vec<String>) -> usize {
    let grid = Grid::from_strings(input);
    let shortest_path = grid.get_shortest_path(
        true,
        grid.get_ending_point(),
        &|n| grid.get(n) == 0,
    );
    return shortest_path.unwrap().len() - 1;
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
        assert!(grid.len() > 0);
        assert!(grid[0].len() > 0);

        let row_length = grid[0].len();

        assert!(grid.iter().all(|x| x.len() == row_length));

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

    pub fn maybe_get(&self, v: Vec2) -> Option<usize> {
        if !self.is_in_bounds(v) {
            return None;
        }
        Some(self.get(v))
    }

    pub fn is_in_bounds(&self, v: Vec2) -> bool {
        let dimensions = self.get_dimensions();
        (v.x() >= 0) && (v.y() >= 0) && (v.x() < dimensions.0 as i32) && (v.y() < dimensions.1 as i32)
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    pub fn get_starting_point(&self) -> Vec2 {
        self.starting_point
    }

    pub fn get_ending_point(&self) -> Vec2 {
        self.ending_point
    }

    pub fn get_coordinates(&self) -> Vec<Vec2> {
        let dimensions = self.get_dimensions();
        (0..dimensions.0).into_iter().map(
            |i| iter::repeat(i).zip(0..dimensions.1).map(|(i, j)| Vec2::new(i as i32, j as i32))
        ).flatten().collect()
    }

    pub fn get_neighbors(&self, coordinates: Vec2) -> HashSet<Vec2> {
        let coordinates_height = self.get(coordinates);
        self.get_adjacent(coordinates).into_iter().filter(|v| self.get(*v) as i32 - coordinates_height as i32 <= 1).collect()
    }

    pub fn get_inverse_neighbors(&self, coordinates: Vec2) -> HashSet<Vec2> {
        let coordinates_height = self.get(coordinates);
        self.get_adjacent(coordinates).into_iter().filter(|v| self.get(*v) as i32 - coordinates_height as i32 >= -1).collect()
    }

    pub fn get_adjacent(&self, coordinates: Vec2) -> HashSet<Vec2> {
        vec![
            coordinates + Vec2::i(),
            coordinates - Vec2::i(),
            coordinates + Vec2::j(),
            coordinates - Vec2::j(),
        ].into_iter().filter(|x| self.is_in_bounds(*x)).collect()
    }

    pub fn get_shortest_path_between(&self, use_inverse_neighbors: bool, start: Vec2, end: Vec2) -> Option<Vec<Vec2>> {
        
        self.get_shortest_path(use_inverse_neighbors, start, &(|n| n == end))

    }

    pub fn get_shortest_path(&self, use_inverse_neighbors: bool, start: Vec2, end_condition: &dyn Fn(Vec2) -> bool) -> Option<Vec<Vec2>> {

        let mut to_check: HashSet<Vec2> = self.get_coordinates().into_iter().collect();
        
        let mut distances_so_far: HashMap<Vec2, usize> = to_check.iter().map(|node| {
            if *node == start {
                (*node, 0)
            } else {
                (*node, usize::MAX)
            }
        }).collect();
        let mut parent_map: HashMap<Vec2, Option<Vec2>> = vec![(start, None)].into_iter().collect();

        loop {
            if to_check.len() == 0 {
                return None;
            }
            let node = *to_check.iter()
                .reduce(|a, i| if distances_so_far.get(i) < distances_so_far.get(a) { i } else {a}).expect("We already checked for empty.");
            if end_condition(node) {
                return Some(Self::get_path_from_parent_map(parent_map, node));
            }
            let neighbors = if use_inverse_neighbors {
                self.get_inverse_neighbors(node)
            } else {
                self.get_neighbors(node)
            };
            let new_distance = distances_so_far.get(&node).expect("We've fully populated distances so far.") + 1;
            for neighbor in neighbors {
                if new_distance < *distances_so_far.get(&neighbor).expect(&format!("We've already populated distances_so_far but {:?} has no entry", neighbor)) {
                    distances_so_far.insert(neighbor, new_distance);
                    parent_map.insert(neighbor, Some(node));
                }
            }
            to_check.remove(&node);
        }

    }

    fn get_path_from_parent_map(parent_map: HashMap<Vec2, Option<Vec2>>, node: Vec2) -> Vec<Vec2> {
        let mut to_return = Vec::new();
        let mut current = node;
        loop {
            to_return.push(current);
            match parent_map.get(&current).expect("The parent map should be complete!") {
                None => return to_return.into_iter().rev().collect(),
                Some(c) => { current = *c; },
            };
        }
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
        assert_eq!(answer, 517);
    }
    
    #[test]
    fn test_problem_12b_passes() {

        assert_eq!(solve_problem_12b(get_example_input()), 29);

        let input = InputParser::new().parse_as_string("input_12.txt").unwrap();

        let answer = solve_problem_12b(input);
        assert_eq!(answer, 512);
    }

    #[test]
    fn test_gets_shortest_path() {
        
        let grid = Grid::from_strings(get_example_input());

        assert_eq!(grid.get_shortest_path_between(false, Vec2::new(0, 0), Vec2::new(0, 0)), Some(vec![Vec2::new(0, 0)]));

        assert_eq!(grid.get_shortest_path_between(false, Vec2::new(0, 0), Vec2::new(0, 1)), Some(vec![Vec2::new(0, 0), Vec2::new(0, 1)]));
        assert_eq!(grid.get_shortest_path_between(false, Vec2::new(0, 0), Vec2::new(1, 0)), Some(vec![Vec2::new(0, 0), Vec2::new(1, 0)]));
        assert_eq!(grid.get_shortest_path_between(false, Vec2::new(1, 7), Vec2::new(0, 7)), Some(vec![Vec2::new(1, 7), Vec2::new(0, 7)]));

        assert_eq!(grid.get_shortest_path_between(false, Vec2::new(0, 0), Vec2::new(1, 1)).unwrap().len(), 3);

        assert_eq!(grid.get_shortest_path_between(false, grid.get_starting_point(), grid.get_ending_point()).unwrap().len(), 32);

    }

    #[test]
    fn test_get_neighbors() {

        let grid = Grid::from_strings(get_example_input());

        assert_eq!(
            grid.get_neighbors(grid.get_starting_point()),
            vec![Vec2::new(0, 1), Vec2::new(1, 0)].into_iter().collect()
        );
        assert_eq!(
            grid.get_neighbors(grid.get_ending_point()),
            vec![Vec2::new(2, 4), Vec2::new(2, 6), Vec2::new(1, 5), Vec2::new(3, 5)].into_iter().collect()
        );
        assert_eq!(
            grid.get_neighbors(Vec2::new(0, 2)),
            vec![Vec2::new(0, 1), Vec2::new(1, 2)].into_iter().collect()
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