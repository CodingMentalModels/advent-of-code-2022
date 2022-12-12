use std::collections::HashSet;

use crate::input::input::InputParser;

pub fn solve_problem_08a(input: Vec<String>) -> usize {
    let forest = Forest::from_strings(input).unwrap();
    forest.count_visible()
}

fn solve_problem_08b(input: Vec<String>) -> usize {
    let forest = Forest::from_strings(input).unwrap();
    forest.get_max_scenic_score()
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<usize>>
}

impl Forest {
    
    pub fn new(trees: Vec<Vec<usize>>) -> Self {
        assert!(trees.len() > 0);
        assert!(trees[0].len() > 0);
        Self {trees}
    }

    pub fn from_strings(strings: Vec<String>) -> Result<Self, String> {
        let trees: Result<Vec<Vec<usize>>, String> = strings.into_iter().map(
            |s| s.chars().map(
                |t| t.to_digit(10).map(|t| t as usize).ok_or("Could't parse tree as digit.".to_string())
            ).collect()
        ).collect();
        trees.map(|t| Self::new(t))
    }

    pub fn n_rows(&self) -> usize {
        self.trees.len()
    }

    pub fn n_columns(&self) -> usize {
        assert!(self.trees.len() > 0);
        self.trees[0].len()
    }

    pub fn get(&self, i: usize, j: usize) -> usize {
        self.trees[i][j]
    }

    pub fn get_row(&self, i: usize) -> Vec<usize> {
        self.trees[i].clone()
    }

    pub fn get_column(&self, j: usize) -> Vec<usize> {
        self.trees.iter().map(|row| row[j]).collect()
    }

    pub fn get_max_scenic_score(&self) -> usize {
        (0..self.n_rows()).into_iter().map(
            |i| (0..self.n_columns()).into_iter().map(
                |j| self.get_scenic_score(i, j)
            ).collect::<Vec<_>>()
        ).flatten().max().expect("At least one element should be in the forest.")
    }

    pub fn get_scenic_score(&self, i: usize, j: usize) -> usize {
        self.get_viewing_distance(Direction::Left, i, j) *
            self.get_viewing_distance(Direction::Right, i, j) *
            self.get_viewing_distance(Direction::Down, i, j) *
            self.get_viewing_distance(Direction::Up, i, j)
    }

    pub fn get_viewing_distance(&self, direction: Direction, i: usize, j: usize) -> usize {
        self.get_view_line(direction, i, j).len()
    }

    pub fn get_view_line(&self, direction: Direction, i: usize, j: usize) -> Vec<(usize, usize)> {

        if direction == Direction::Left && j == 0 { return Vec::new() }
        if direction == Direction::Right && j == self.n_columns() - 1 { return Vec::new() }
        if direction == Direction::Down && i == self.n_rows() - 1 { return Vec::new() }
        if direction == Direction::Up && i == 0 { return Vec::new() }

        let trees_to_check: Vec<(usize, usize)> = match direction {
            Direction::Left => (0..j).into_iter().map(|y| (i, y)).rev().collect(),
            Direction::Right => ((j + 1)..(self.n_columns())).into_iter().map(|y| (i, y)).collect(),
            Direction::Down => ((i + 1)..(self.n_rows())).into_iter().map(|x| (x, j)).collect(),
            Direction::Up => (0..i).into_iter().map(|x| (x, j)).rev().collect(),
        };

        let mut to_return: Vec<(usize, usize)> = Vec::new();
        let height_to_match = self.get(i, j);
        for tree in trees_to_check {
            to_return.push(tree);
            if self.get(tree.0, tree.1) >= height_to_match {
                return to_return;
            }
        }
        return to_return;
    }

    pub fn count_visible(&self) -> usize {
        self.get_visible().len()
    }

    pub fn get_visible(&self) -> HashSet<(usize, usize)> {
        let boundary = self.get_boundary_coordinates();
        let visible_in_rows: HashSet<(usize, usize)> = (0..self.n_rows()).into_iter().map(|i| self.get_visible_in_row(i)).flatten().collect();
        let visible_in_columns: HashSet<(usize, usize)> = (0..self.n_columns()).into_iter().map(|j| self.get_visible_in_column(j)).flatten().collect();
        return boundary.union(&visible_in_rows).cloned().collect::<HashSet<_>>().union(&visible_in_columns).cloned().collect();
    }

    pub fn get_boundary_coordinates(&self) -> HashSet<(usize, usize)> {
        let left: HashSet<(usize, usize)> = (0..self.n_rows()).into_iter().map(|i| (i, 0)).collect();
        let right: HashSet<(usize, usize)> = (0..self.n_rows()).into_iter().map(|i| (i, self.n_columns() - 1)).collect();
        let top: HashSet<(usize, usize)> = (0..self.n_columns()).into_iter().map(|j| (0, j)).collect();
        let bottom: HashSet<(usize, usize)> = (0..self.n_columns()).into_iter().map(|j| (self.n_rows() - 1, j)).collect();
        
        return left.union(&right).cloned().collect::<HashSet<_>>().union(&top).cloned().collect::<HashSet<_>>().union(&bottom).cloned().collect();
    }

    pub fn get_visible_in_row(&self, i: usize) -> HashSet<(usize, usize)> {
        Self::get_visible_from_list(self.get_row(i)).into_iter().map(|n| (i, n)).collect()
    }

    pub fn get_visible_in_column(&self, j: usize) -> HashSet<(usize, usize)> {
        Self::get_visible_from_list(self.get_column(j)).into_iter().map(|n| (n, j)).collect()
    }

    fn get_visible_from_list(trees: Vec<usize>) -> Vec<usize> {
        let length = trees.len();
        let from_front: HashSet<usize> = Self::get_visible_from_front(trees.clone()).into_iter().collect();
        let from_back: HashSet<usize> = Self::get_visible_from_front(trees.into_iter().rev().collect()).into_iter().map(|n| length - n - 1).collect();
        return from_front.union(&from_back).map(|x| x.clone()).collect();
    }

    fn get_visible_from_front(trees: Vec<usize>) -> Vec<usize> {
        let mut tallest_so_far = 0;
        let mut to_return = Vec::new();
        for (i, tree) in trees.iter().enumerate() {
            if *tree > tallest_so_far {
                tallest_so_far = *tree;
                to_return.push(i);
            }
        }
        return to_return;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {

    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Down => (0, -1),
            Self::Up => (0, 1),
        }
    }
}

#[cfg(test)]
mod test_problem_08 {

    use super::*;

    #[test]
    fn test_problem_08a_passes() {
        
        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        assert_eq!(solve_problem_08a(example_input), 21);

        let input = InputParser::new().parse_as_string("input_08.txt").unwrap();

        let answer = solve_problem_08a(input);
        assert_eq!(answer, 1676);
    }
    
    #[test]
    fn test_problem_08b_passes() {

        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        assert_eq!(solve_problem_08b(example_input), 8);

        let input = InputParser::new().parse_as_string("input_08.txt").unwrap();

        let answer = solve_problem_08b(input);
        assert_eq!(answer, 313200);
    }

    #[test]
    fn test_forest_computes_visible_trees() {
        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let forest = Forest::from_strings(example_input).unwrap();
        let expected = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),

            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),

            (1, 0),
            (2, 0),
            (3, 0),

            (1, 4),
            (2, 4),
            (3, 4),

            (1, 1),
            (1, 2),
            (2, 1),
            (2, 3),
            (3, 2),
        ].into_iter().collect();
        let visible = forest.get_visible();
        assert_eq!(
            visible.clone(),
            expected,
            "Extra visible: {:?}\nExtra Expected: {:?}", visible.clone().difference(&expected), expected.difference(&visible.clone())
        );
        assert_eq!(forest.count_visible(), 21);
    }

    #[test]
    fn test_forest_gets_visible_trees() {
        
        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let forest = Forest::from_strings(example_input).unwrap();
        assert_eq!(
            forest.get_visible_in_row(1),
            vec![(1, 0), (1, 1), (1, 2), (1, 4)].into_iter().collect(),
        );
        assert_eq!(
            forest.get_visible_in_column(3),
            vec![(0, 3), (4, 3)].into_iter().collect(),
        );
        assert_eq!(
            forest.get_visible_in_column(2),
            vec![(0, 2), (1, 2), (3, 2), (4, 2)].into_iter().collect()
        );
    }

    #[test]
    fn test_forest_gets_tree() {
        
        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let forest = Forest::from_strings(example_input).unwrap();
        assert_eq!(forest.get(0, 0), 3);
        assert_eq!(forest.get(4, 4), 0);
        assert_eq!(forest.get(3, 2), 5);
        
        assert_eq!(forest.get_row(3), vec![3, 3, 5, 4, 9]);
        assert_eq!(forest.get_column(1), vec![0, 5, 5, 3, 5]);
    }

    #[test]
    fn test_gets_scene_score_of_tree() {
        
        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let forest = Forest::from_strings(example_input).unwrap();
        assert_eq!(forest.get_scenic_score(1, 2), 4);
        assert_eq!(forest.get_scenic_score(3, 2), 8);
    }

    #[test]
    fn test_viewing_distance() {
        
        let example_input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let forest = Forest::from_strings(example_input).unwrap();
        assert!((0..forest.n_rows()).into_iter().all(|i| forest.get_viewing_distance(Direction::Left, i, 0) == 0));
        assert!((0..forest.n_columns()).into_iter().all(|j| forest.get_viewing_distance(Direction::Down, 4, j) == 0));
        assert_eq!(forest.get_viewing_distance(Direction::Up, 1, 2), 1);
        assert_eq!(forest.get_viewing_distance(Direction::Right, 2, 0), 4);
    }

}