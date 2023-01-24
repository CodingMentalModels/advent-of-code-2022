use std::collections::{HashMap, HashSet};
use std::iter::Repeat;

use crate::input::input::InputParser;
use crate::utility::direction::Direction;
use crate::utility::vector::Vec2;

pub fn solve_problem_17a(input: String) -> usize {

    let mut simulation = Simulation::from_string(input);
    simulation.step_until_before_the_nth_rock(2023);

    simulation.get_height()
}

fn solve_problem_17b(input: String) -> usize {

    let mut simulation = Simulation::from_string(input);
    simulation.step_until_before_the_nth_rock(1_000_000_000_001);

    simulation.get_height()

}

type Time = usize;

const WIDTH: usize = 7;
const MAX_PERIOD: usize = 50455;

struct Simulation {
    jet_patterns: Vec<Direction>,
    time_elapsed: Time,
    n_rocks_fallen: usize,
    simulation_phase: SimulationPhase,
    falling_rock: Rock,
    next_jet_pattern_idx: usize,
    occupied_squares: OccupiedSquares,
}

impl Simulation {

    pub fn new(jet_patterns: Vec<Direction>) -> Self {
        let initial_rock = Rock::Square;
        Self {
            jet_patterns,
            time_elapsed: 0,
            n_rocks_fallen: 0,
            simulation_phase: SimulationPhase::NewRock,
            falling_rock: initial_rock,
            next_jet_pattern_idx: 0,
            occupied_squares: OccupiedSquares::default(),
        }
    }

    pub fn from_string(jet_patterns: String) -> Self {
        Self::new(
            jet_patterns.chars().filter(|c| *c == '<' || *c == '>').map(|c| {
                match c {
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!("We filtered down to just these already.")
                }
            }).collect()
        )
    }

    pub fn get_time_elapsed(&self) -> Time {
        self.time_elapsed
    }

    pub fn get_n_rocks_fallen(&self) -> usize {
        self.n_rocks_fallen
    }

    pub fn get_falling_rock_position(&self) -> Option<Vec2> {
        self.simulation_phase.get_rock_position()
    }

    pub fn get_falling_rock_type(&self) -> Option<Rock> {
        match self.simulation_phase {
            SimulationPhase::NewRock => {
                None
            },
            _ => { Some(self.falling_rock) }
        }
    }

    pub fn get_occupied_squares(&self) -> &OccupiedSquares {
        &self.occupied_squares
    }

    pub fn get_period(&mut self) -> Option<Time> {
        let mut counter = 0;
        while self.get_n_rocks_fallen() == 0 || !self.is_flat() || self.falling_rock != Rock::Minus || self.next_jet_pattern_idx != 0 {
            if counter > MAX_PERIOD {
                return None;
            }
            self.step_until_rock_lands();
            counter += 1;
        }

        return Some(self.get_time_elapsed());
    }

    pub fn is_flat(&self) -> bool {
        if self.occupied_squares.is_empty() {
            return true;
        }
        let max_y = self.occupied_squares.get_max_y();
        (0..WIDTH).into_iter().all(|x| self.occupied_squares.contains(Vec2::new(x as i32, max_y as i32)))
    }

    pub fn get_height(&self) -> usize {
        if self.occupied_squares.is_empty() {
            return 0;
        }
        self.occupied_squares.get_max_y() + 1
    }

    pub fn step_until_before_the_nth_rock(&mut self, n: usize) {
        assert!(n > 0);
        for _i in (0..(n - 1)) {
            self.step_until_rock_lands()
        }
    }

    pub fn step_until_rock_lands(&mut self) {
        loop {
            self.step();

            if self.simulation_phase == SimulationPhase::NewRock {
                return;
            }
        }
    }

    pub fn step(&mut self) {
        self.time_elapsed += 1;
        match self.simulation_phase {
            SimulationPhase::NewRock => {
                self.falling_rock = self.falling_rock.get_next();
                self.n_rocks_fallen += 1;

                self.simulation_phase = SimulationPhase::HandleJet(self.get_falling_rock_initial_position());
            },
            SimulationPhase::HandleJet(_rock_position) => {
                let jet_direction = self.jet_patterns[self.next_jet_pattern_idx];
                self.next_jet_pattern_idx = (self.next_jet_pattern_idx + 1) % self.jet_patterns.len();

                let new_position = self.get_movement_effect(jet_direction);
                self.simulation_phase = SimulationPhase::HandleFall(new_position);
            },
            SimulationPhase::HandleFall(rock_position) => {
                if self.falling_rock_has_landed() {
                    self.occupied_squares = self.occupied_squares.union(&self.falling_rock.get_stone_positions(rock_position));
                    self.simulation_phase = SimulationPhase::NewRock;
                } else {
                    let new_position = self.get_movement_effect(Direction::Down);
                    self.simulation_phase = SimulationPhase::HandleJet(new_position);
                }
            }
        }

    }

    pub fn falling_rock_has_landed(&self) -> bool {
        self.get_movement_effect(Direction::Down) == self.simulation_phase.get_rock_position().unwrap()
    }

    fn get_movement_effect(&self, direction: Direction) -> Vec2 {
        assert_ne!(direction, Direction::Up);

        let original_position = self.simulation_phase.get_rock_position()
            .expect("We shouldn't be checking whether the rock has landed if it's a new rock.");

        let potential_new_position = original_position + direction.get_delta();

        if direction == Direction::Down {
            if original_position.y() == 0 {
                return original_position;
            } else if self.falling_rock.get_stone_positions(potential_new_position).is_disjoint(&self.occupied_squares) {
                return potential_new_position;
            } else {
                return original_position;
            }
        }
        
        if self.falling_rock.rock_would_collide_with_wall(potential_new_position) {
            return original_position;
        } 

        let rock_squares = self.falling_rock.get_stone_positions(potential_new_position);

        if !rock_squares.is_disjoint(&self.occupied_squares) {
            return original_position;
        } else {
            return potential_new_position;
        }
        
    }

    fn get_falling_rock_initial_position(&self) -> Vec2 {
        Vec2::new(2, self.get_height() as i32 + 3)
    }


}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum SimulationPhase {
    NewRock,
    HandleJet(Vec2),
    HandleFall(Vec2),
}

impl SimulationPhase {

    pub fn get_rock_position(&self) -> Option<Vec2> {
        match self {
            Self::NewRock => None,
            Self::HandleFall(v) => Some(v.clone()),
            Self::HandleJet(v) => Some(v.clone()),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Rock {
    Minus,
    Plus,
    BackwardsL,
    Bar,
    Square,
}

impl Rock {

    pub fn all() -> Vec<Self> {
        vec![
            Self::Minus,
            Self::Plus,
            Self::BackwardsL,
            Self::Bar,
            Self::Square,
        ]
    }

    pub fn get_next(&self) -> Self {
        match self {
            Self::Minus => { Self::Plus },
            Self::Plus => { Self::BackwardsL },
            Self::BackwardsL => { Self::Bar },
            Self::Bar => { Self::Square },
            Self::Square => { Self::Minus },
        }
    }

    pub fn get_width(&self) -> usize {
        match self {
            Self::Minus => { 4 },
            Self::Plus => { 3 },
            Self::BackwardsL => { 3 },
            Self::Bar => { 1 },
            Self::Square => { 2 },
        }
    }

    pub fn rock_would_collide_with_wall(&self, rock_position: Vec2) -> bool {
        (rock_position.x() < 0) || (rock_position.x() + self.get_width() as i32 > WIDTH as i32)
    }

    pub fn get_stone_positions(&self, rock_position: Vec2) -> OccupiedSquares {
        assert!(!self.rock_would_collide_with_wall(rock_position));
        self.get_relative_stone_positions().shift(rock_position)
    }

    pub fn get_relative_stone_positions(&self) -> OccupiedSquares {
        OccupiedSquares::new(Self::get_relative_stone_positions_as_vecs(self))
    }

    fn get_relative_stone_positions_as_vecs(&self) -> HashSet<Vec2> {
        match self {
            Self::Minus => vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(2, 0),
                Vec2::new(3, 0),
            ],
            Self::Plus => vec![
                Vec2::new(0, 1),
                Vec2::new(1, 0),
                Vec2::new(1, 1),
                Vec2::new(1, 2),
                Vec2::new(2, 1),
            ],
            Self::BackwardsL => vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(2, 0),
                Vec2::new(2, 1),
                Vec2::new(2, 2),
            ],
            Self::Bar => vec![
                Vec2::new(0, 0),
                Vec2::new(0, 1),
                Vec2::new(0, 2),
                Vec2::new(0, 3),
            ],
            Self::Square => vec![
                Vec2::new(0, 0),
                Vec2::new(0, 1),
                Vec2::new(1, 0),
                Vec2::new(1, 1),
            ],

        }.into_iter().collect()
    }

}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct OccupiedSquares(HashSet<Vec2>);

impl OccupiedSquares {

    pub fn new_unchecked(vecs: HashSet<Vec2>) -> Self {
        Self(vecs)
    }

    pub fn new(vecs: HashSet<Vec2>) -> Self {
        Self::new_unchecked(vecs)
        // Self(vecs).get_surface()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersect(other).is_empty()
    }

    pub fn intersects(&self, other: &Self) -> bool {
        !self.is_disjoint(other)
    }

    pub fn contains(&self, v: Vec2) -> bool {
        self.intersects(&Self::new_unchecked(vec![v].into_iter().collect()))
    }

    pub fn intersect(&self, other: &Self) -> Self {
        Self(self.0.intersection(&other.0).cloned().collect())
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.union(&other.0).cloned().collect())
    }

    pub fn get_max_y(&self) -> usize {
        self.0.iter().map(|v| v.y()).max().unwrap_or(0) as usize
    }

    pub fn shift(&self, delta_v: Vec2) -> Self {
        Self::new(self.0.iter().map(|v| v.clone() + delta_v).collect())
    }

    fn get_surface(&self) -> Self {
        let mut to_return = HashSet::new();
        let mut surface_element = self.0.iter()
            .filter(|v| v.x() == 0)
            .map(|v| v.y()).max()
            .map(|max_y| Vec2::new(0, max_y));
        let mut x = 0;
        while x < WIDTH {
            if surface_element.is_some() {
                to_return.insert(surface_element.unwrap().clone());
            }
            let (next_x, next_surface_element) = self.get_next_surface_element(x, surface_element);
            x = next_x;
            surface_element = next_surface_element;
        }
        return Self::new_unchecked(to_return);
    }

    fn get_next_surface_element(&self, x: usize, surface_element: Option<Vec2>) -> (usize, Option<Vec2>) {
        if surface_element.is_some() {
            assert_eq!(surface_element.unwrap().x(), x as i32);
        }
        let y_plus_one = surface_element.map(|v| v.y() + 1).unwrap_or(0);

        let above = Vec2::new(x as i32, y_plus_one);
        let above_and_left = above - Vec2::i();
        if self.contains(above_and_left) {
            return (above_and_left.x() as usize, Some(above_and_left));
        }

        if self.contains(above) {
            return (x, Some(above));
        }

        let above_and_right = above + Vec2::i();
        if self.contains(above_and_right) {
            return (x + 1, Some(above_and_right));
        }

        if surface_element.is_none() {
            return (x + 1, None)
        }
        
        let v = surface_element.unwrap();

        let right = v + Vec2::i();
        if self.contains(right) {
            return (x + 1, Some(right));
        }

        let mut right_and_down = right - Vec2::j();
        while right_and_down.y() >= 0 {
            if self.contains(right_and_down) {
                return (x + 1, Some(right_and_down))
            }
            right_and_down = right_and_down - Vec2::j();
        }

        return (x + 1, None);

    }

}

#[cfg(test)]
mod test_problem_17 {

    use std::iter;

    use super::*;

    fn get_example_input() -> String {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string()
    }

    #[test]
    fn test_problem_17a_passes() {

        assert_eq!(solve_problem_17a(get_example_input()), 3068);
        
        let input = InputParser::new().parse_to_single_string("input_17.txt").unwrap();

        let answer = solve_problem_17a(input);
        assert_eq!(answer, 3215);
    }
    
    #[test]
    fn test_problem_17b_passes() {
        
        assert_eq!(solve_problem_17b(get_example_input()), 1514285714288);

        let input = InputParser::new().parse_to_single_string("input_17.txt").unwrap();

        let answer = solve_problem_17b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_detects_cycles() {

        let mut simulation = Simulation::from_string(get_example_input());

        assert_eq!(simulation.get_period(), Some(0));
    }

    #[test]
    fn test_simulates() {
        
        let mut simulation = Simulation::from_string(get_example_input());

        assert_eq!(simulation.get_n_rocks_fallen(), 0);
        assert_eq!(simulation.get_falling_rock_type(), None);
        assert_eq!(simulation.get_height(), 0);

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_type(), Some(Rock::Minus));
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(2, 3)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(3, 3)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(3, 2)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(3, 2)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(3, 1)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(3, 1)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(3, 0)));
        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(2, 0)));

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 1);
        assert_eq!(simulation.get_falling_rock_position(), None);
        assert_eq!(
            simulation.get_occupied_squares().clone(),
            OccupiedSquares::new(vec![Vec2::new(2, 0), Vec2::new(3, 0), Vec2::new(4, 0), Vec2::new(5, 0)].into_iter().collect())
        );
        assert_eq!(simulation.get_time_elapsed(), 9);
        assert_eq!(simulation.get_falling_rock_type(), None);
        assert_eq!(simulation.get_height(), 1);

        simulation.step();

        assert_eq!(simulation.get_n_rocks_fallen(), 2);
        assert_eq!(simulation.get_falling_rock_position(), Some(Vec2::new(2, 4)));
        assert_eq!(simulation.get_time_elapsed(), 10);
        assert_eq!(simulation.get_falling_rock_type(), Some(Rock::Plus));
        assert_eq!(simulation.get_height(), 1);

        simulation.step_until_rock_lands();
        
        assert_eq!(simulation.get_n_rocks_fallen(), 2);
        assert_eq!(simulation.get_occupied_squares().len(), 9, "{:?}", simulation.get_occupied_squares());
        assert!(simulation.get_occupied_squares().contains(Vec2::new(3, 3)));
        assert!(!simulation.get_occupied_squares().contains(Vec2::new(2, 1)));
        assert!(!simulation.get_occupied_squares().contains(Vec2::new(2, 3)));
        assert_eq!(simulation.get_falling_rock_type(), None);
        assert_eq!(simulation.get_height(), 4);

    }

    #[test]
    fn test_rock_lands() {

        let mut simulation = Simulation::from_string(get_example_input());

        simulation.occupied_squares = OccupiedSquares::new(vec![
            Vec2::new(3, 0),
            Vec2::new(3, 1),
            Vec2::new(3, 2),
            Vec2::new(3, 3),
            Vec2::new(3, 4),
        ].into_iter().collect());

        // simulation.simulation_phase = SimulationPhase::HandleFall(Vec2::new(3, 6));
        // assert!(!simulation.falling_rock_has_landed());
        
        // simulation.simulation_phase = SimulationPhase::HandleFall(Vec2::new(3, 5));
        // assert!(simulation.falling_rock_has_landed());
        
    }

}