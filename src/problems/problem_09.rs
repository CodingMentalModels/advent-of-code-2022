use std::{collections::HashSet, iter};

use crate::{input::input::InputParser, utility::{vector::Vec2, direction::Direction}};

pub fn solve_problem_09a(input: Vec<String>) -> usize {
    let mut rope = RopeLink::new();
    
    rope.make_moves_and_get_tails(input.into_iter().map(|s| Move::from_string(s).unwrap()).collect())
        .into_iter().collect::<HashSet<_>>().len()
}

fn solve_problem_09b(input: Vec<String>) -> usize {
    let mut rope = Rope::default(9);

    rope.make_moves_and_get_tails(input.into_iter().map(|s| Move::from_string(s).unwrap()).collect())
        .into_iter().collect::<HashSet<_>>().len()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rope {
    links: Vec<RopeLink>
}

impl Rope {

    pub fn new(links: Vec<RopeLink>) -> Self {
        assert!(links.len() > 0);
        Self {links}
    }

    pub fn default(n_links: usize) -> Self {
        assert!(n_links > 0);
        Self::new(iter::repeat(RopeLink::default()).take(n_links).collect())
    }

    pub fn make_moves_and_get_tails(&mut self, moves: Vec<Move>) -> Vec<Vec2> {
        moves.into_iter().map(|m| self.make_move_and_get_tails(m)).flatten().collect()
    }

    pub fn make_move_and_get_tails(&mut self, m: Move) -> Vec<Vec2> {
        let (direction, repetitions) = m.unpack();
        iter::repeat(direction).take(repetitions).map(|d| self.step_and_get_tail(d)).collect()
    }

    fn step_and_get_tail(&mut self, direction: Direction) -> Vec2 {
        let mut last_tail = self.links[0].move_in_direction_and_get_tail(direction);
        for i in (1..self.links.len()) {
            last_tail = self.links[i].update_head_and_follow(last_tail);
        }
        return last_tail;
    }

}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct RopeLink {
    head: Vec2,
    tail: Vec2,
}

impl RopeLink {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn make_moves_and_get_tails(&mut self, moves: Vec<Move>) -> Vec<Vec2> {
        moves.into_iter().map(
            |m| self.make_move_and_get_tails(m)
        ).flatten().collect()
    }

    pub fn make_move_and_get_tails(&mut self, m: Move) -> Vec<Vec2> {
        let (direction, n) = m.unpack();
        (0..n).into_iter().map(|_i| self.move_in_direction_and_get_tail(direction)).collect()
    }

    fn move_in_direction_and_get_tail(&mut self, direction: Direction) -> Vec2 {
        self.head += direction.get_delta();
        self.update_tail()
    }

    pub fn update_head_and_follow(&mut self, new_head: Vec2) -> Vec2 {
        assert!((new_head - self.head).get_l1_norm() <= 2);
        self.head = new_head;
        let tail = self.update_tail();
        assert!(self.get_l1_length() <= 2);
        return tail;
    }

    fn update_tail(&mut self) -> Vec2 {
        assert!(self.get_l1_length() <= 4);
        if self.get_l1_length() <= 1 {
            return self.tail;
        } else if self.get_l1_length() == 2 {
            if self.is_colinear() {
                self.tail += self.get_direction_vector();
                return self.tail;
            } else {
                return self.tail;
            }
        } else {
            assert!(self.get_l1_length() == 3 || self.get_l1_length() == 4);
            let direction_vector = self.get_direction_vector();
            self.tail += direction_vector;
            return self.tail
        }
    }

    pub fn get_l1_length(&self) -> u32 {
        (self.head - self.tail).get_l1_norm()
    }

    pub fn get_direction_vector(&self) -> Vec2 {
        (self.head - self.tail).signum()
    }

    pub fn is_colinear(&self) -> bool {
        self.head.x() == self.tail.x() || self.head.y() == self.tail.y()
    }

}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Move {
    direction: Direction,
    repetitions: usize,
}

impl Move {
    
    pub fn new(direction: Direction, repetitions: usize) -> Self {
        Self {direction, repetitions}
    }

    pub fn from_string(s: String) -> Result<Self, String> {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        assert_eq!(parts.len(), 2);
        let (direction, repetitions) = (
            Direction::from_string(parts[0])?,
            parts[1].parse::<usize>().map_err(|_| format!("Unable to parse repetitions: {:?}", parts[1]))?
        );
        Ok(Self::new(direction, repetitions))
    }

    pub fn unpack(&self) -> (Direction, usize) {
        (self.direction, self.repetitions)
    }
}
#[cfg(test)]
mod test_problem_09 {

    use std::collections::HashSet;

    use super::*;

    fn get_example_09a_input() -> Vec<String> {
        vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ]
    }

    fn get_example_09b_input() -> Vec<String> {
        vec![
            "R 5".to_string(),
            "U 8".to_string(),
            "L 8".to_string(),
            "D 3".to_string(),
            "R 17".to_string(),
            "D 10".to_string(),
            "L 25".to_string(),
            "U 20".to_string(),
        ]
    }

    #[test]
    fn test_problem_09a_passes() {
        
        let example = get_example_09a_input();
        assert_eq!(solve_problem_09a(example), 13);

        let input = InputParser::new().parse_as_string("input_09.txt").unwrap();

        let answer = solve_problem_09a(input);
        assert_eq!(answer, 5878);
    }

    #[test]
    fn test_problem_09b_passes() {
        let example_input = get_example_09b_input();
        assert_eq!(solve_problem_09b(example_input), 36);

        let input = InputParser::new().parse_as_string("input_09.txt").unwrap();

        let answer = solve_problem_09b(input);
        assert_eq!(answer, 2405);
    }

    #[test]
    fn test_rope_moves_several() {
        let example_input = get_example_09b_input();
        let mut rope = Rope::default(9);

        let tail_positions = rope.make_moves_and_get_tails(example_input.into_iter().map(|s| Move::from_string(s).unwrap()).collect());
        assert_eq!(tail_positions.into_iter().collect::<HashSet<_>>().len(), 36);
    }

    #[test]
    fn test_rope_moves() {
        let mut trivial_rope = Rope::default(1);

        assert_eq!(*trivial_rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), Vec2::new(0, 0));
        assert_eq!(*trivial_rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), Vec2::new(0, 1));
        assert_eq!(*trivial_rope.make_move_and_get_tails(Move::new(Direction::Up, 2)).last().unwrap(), Vec2::new(0, 3));
        assert_eq!(*trivial_rope.make_move_and_get_tails(Move::new(Direction::Down, 1)).last().unwrap(), Vec2::new(0, 3));
        assert_eq!(*trivial_rope.make_move_and_get_tails(Move::new(Direction::Right, 1)).last().unwrap(), Vec2::new(0, 3));
        assert_eq!(*trivial_rope.make_move_and_get_tails(Move::new(Direction::Up, 2)).last().unwrap(), Vec2::new(1, 4));

        let mut rope = Rope::default(10);
        assert_eq!(rope.make_move_and_get_tails(Move::new(Direction::Right, 5)).last().unwrap(), &Vec2::new(0, 0));
        assert_eq!(
            rope.make_move_and_get_tails(Move::new(Direction::Up, 8)),
            vec![
                Vec2::new(0, 0),
                Vec2::new(0, 0),
                Vec2::new(0, 0),
                Vec2::new(0, 0),
                Vec2::new(0, 0),
                Vec2::new(0, 0),
                Vec2::new(0, 0),
                Vec2::new(0, 0),
            ]
        );

        let mut rope = Rope::default(3);
        assert_eq!(rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), &Vec2::new(0, 0)); // Head at (0, 1)
        assert_eq!(rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), &Vec2::new(0, 0)); // Head at (0, 2)
        assert_eq!(rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), &Vec2::new(0, 0)); // Head at (0, 3)
        assert_eq!(rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), &Vec2::new(0, 1)); // Head at (0, 4)
        
        assert_eq!(rope.make_move_and_get_tails(Move::new(Direction::Right, 4)).last().unwrap(), &Vec2::new(2, 3)); // Head at (4, 4)

    }
    

    #[test]
    fn test_rope_link_moves_several() {
        
        let example_input = get_example_09a_input();

        let mut rope = RopeLink::new();
        let tail_positions = rope.make_moves_and_get_tails(example_input.into_iter().map(|s| Move::from_string(s).unwrap()).collect())
            .into_iter().collect::<HashSet<_>>();
        assert_eq!(
            tail_positions,
            vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(2, 0),
                Vec2::new(3, 0),
                Vec2::new(4, 1),
                Vec2::new(1, 2),
                Vec2::new(2, 2),
                Vec2::new(3, 2),
                Vec2::new(4, 2),
                Vec2::new(3, 3),
                Vec2::new(4, 3),
                Vec2::new(2, 4),
                Vec2::new(3, 4),
            ].into_iter().collect()
        );
        assert_eq!(tail_positions.len(), 13);

    }

    #[test]
    fn test_rope_link_moves() {
        let mut rope = RopeLink::new();
        assert_eq!(*rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), Vec2::new(0, 0));
        assert_eq!(*rope.make_move_and_get_tails(Move::new(Direction::Up, 1)).last().unwrap(), Vec2::new(0, 1));
        assert_eq!(*rope.make_move_and_get_tails(Move::new(Direction::Up, 2)).last().unwrap(), Vec2::new(0, 3));
        assert_eq!(*rope.make_move_and_get_tails(Move::new(Direction::Down, 1)).last().unwrap(), Vec2::new(0, 3));
        assert_eq!(*rope.make_move_and_get_tails(Move::new(Direction::Right, 1)).last().unwrap(), Vec2::new(0, 3));
        assert_eq!(*rope.make_move_and_get_tails(Move::new(Direction::Up, 2)).last().unwrap(), Vec2::new(1, 4));
    }

}