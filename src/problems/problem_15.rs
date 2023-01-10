use std::collections::HashSet;

use crate::{input::input::InputParser, utility::{vector::Vec2, parser::Parser}};

pub fn solve_problem_15a(input: Vec<String>, row_y: i32) -> usize {
    let sensors = Sensor::from_strings(input);
    let (min_x, max_x) = sensors.iter().map(|s| s.get_x_bounds()).reduce(|mut accumulator, element| {
            let (l, r) = element;
            accumulator.0 = if l < accumulator.0 { l } else { accumulator.0 };
            accumulator.1 = if r > accumulator.1 { r } else { accumulator.1 };
            accumulator
        }
    ).expect("There's at least one sensor.");
    let impossible_positions = (min_x..=max_x).into_iter()
        .filter(
            |x| sensors.iter().any(
                |sensor| !sensor.beacon_is_possible(Vec2::new(*x, row_y))
            )
        ).collect::<Vec<_>>();
    return impossible_positions.len();
}

fn solve_problem_15b(input: Vec<String>, search_space_size: usize) -> u64 {
    let sensors = Sensor::from_strings(input);
    for s in sensors.iter() {
        let constraint = |v: Vec2| {
            0 <= v.x() &&
            v.x() <= search_space_size as i32 &&
            0 <= v.y() &&
            v.y() <= search_space_size as i32
        };
        let surface = s.get_surface_such_that(&constraint);
        for position in surface.iter() {
            // Because the point is unique, it's guaranteed to be on the "surface" of one of the sensors.
            // Proof: Suppose it's not on the surface, then there are no sensors within 1 of it, but that means
            // that moving one over, you get another answer, violating uniqueness.
            let hit = sensors.iter().all(
                |sensor| sensor.beacon_is_possible(*position) && sensor.beacon_position != *position
            );
            if hit {
                return (4_000_000_u64 * (position.x() as u64) + (position.y() as u64)).try_into().unwrap();
            }
        }
    }
    panic!()
}

struct Sensor {
    position: Vec2,
    beacon_position: Vec2,
    max_beacon_distance: u32,
}

impl Sensor {

    pub fn new(position: Vec2, beacon_position: Vec2) -> Self {
        let max_beacon_distance = (position - beacon_position).get_l1_norm();
        Self { position, beacon_position, max_beacon_distance }
    }

    pub fn from_strings(strings: Vec<String>) -> Vec<Self> {
        let mut to_return: Vec<Self> = strings.into_iter().map(|s| Self::from_string(s)).collect();
        to_return.sort_by(|a, b| a.max_beacon_distance.cmp(&b.max_beacon_distance).reverse());
        return to_return;
    }

    pub fn from_string(s: String) -> Self {

        let mut parser = Parser::new(s);

        let _sensor_header = parser.expect("Sensor at ").unwrap();
        let sensor_position = Self::parse_vec2(&mut parser);

        let _closest_beacon_header = parser.expect(": closest beacon is at ");
        let beacon_position = Self::parse_vec2(&mut parser);

        return Self::new(sensor_position, beacon_position);

    }

    fn parse_vec2(parser: &mut Parser) -> Vec2 {
        let _x_equals = parser.expect("x=").unwrap();
        let x = parser.consume_until(&|c| c == ',').parse::<i32>().expect("x should be parsable.");
        let _comma_space_y_equals = parser.expect(", y=").unwrap();
        let y = parser.consume_until(&|c| c == ':' || c == '\n').parse::<i32>().expect("y should be parsable.");
        return Vec2::new(x, y);
    }

    pub fn beacon_is_possible(&self, position: Vec2) -> bool {
        if position == self.beacon_position {
            return true;
        }
        (position - self.position).get_l1_norm() > self.max_beacon_distance
    }

    pub fn get_x_bounds(&self) -> (i32, i32) {
        (self.position.x() - self.max_beacon_distance as i32, self.position.x() + self.max_beacon_distance as i32)
    }

    pub fn get_y_bounds(&self) -> (i32, i32) {
        (self.position.y() - self.max_beacon_distance as i32, self.position.y() + self.max_beacon_distance as i32)
    }
    
    pub fn get_surface_such_that(&self, constraint: &dyn Fn(Vec2) -> bool) -> HashSet<Vec2> {
        let r = (self.max_beacon_distance + 1) as i32;
        let right = self.position + Vec2::i() * r;
        let left = self.position - Vec2::i() * r;
        let bottom = self.position + Vec2::j() * r;
        let top = self.position - Vec2::j() * r;

        let top_left_quadrant = Vec2::get_points_between(left, top).into_iter().filter(|v| constraint(*v)).collect::<HashSet<_>>();
        let top_right_quadrant = Vec2::get_points_between(top, right).into_iter().filter(|v| constraint(*v)).collect::<HashSet<_>>();
        let bottom_left_quadrant = Vec2::get_points_between(bottom, left).into_iter().filter(|v| constraint(*v)).collect::<HashSet<_>>();
        let bottom_right_quadrant = Vec2::get_points_between(bottom, right).into_iter().filter(|v| constraint(*v)).collect::<HashSet<_>>();

        return top_left_quadrant
            .union(&top_right_quadrant).cloned().collect::<HashSet<_>>()
            .union(&bottom_left_quadrant).cloned().collect::<HashSet<_>>()
            .union(&bottom_right_quadrant).cloned().collect::<HashSet<_>>();
    }

}

#[cfg(test)]
mod test_problem_15 {

    use super::*;

    fn get_example_input() -> Vec<String> {
        InputParser::new().parse_as_string("example_input_15.txt").unwrap()
    }

    #[test]
    fn test_problem_15a_passes() {

        let example_input = get_example_input();
        assert_eq!(solve_problem_15a(example_input, 10), 26);
        
        let input = InputParser::new().parse_as_string("input_15.txt").unwrap();
        
        let answer = solve_problem_15a(input, 2_000_000);
        assert_eq!(answer, 4811413);
    }
    
    #[test]
    fn test_problem_15b_passes() {

        let example_input = get_example_input();
        assert_eq!(solve_problem_15b(example_input, 20), 56_000_011);

        let input = InputParser::new().parse_as_string("input_15.txt").unwrap();

        let answer = solve_problem_15b(input, 4_000_000);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_sensors_query() {
        
        let sensors = Sensor::from_strings(get_example_input());
        assert_eq!(sensors.len(), 14);

        assert!(!sensors[1].beacon_is_possible(Vec2::new(8, 7)));
        assert!(!sensors[1].beacon_is_possible(Vec2::new(8, 8)));
        assert!(!sensors[1].beacon_is_possible(Vec2::new(7, 7)));
        assert!(!sensors[1].beacon_is_possible(Vec2::new(-1, 7)));
        assert!(!sensors[1].beacon_is_possible(Vec2::new(8, -2)));

        assert!(sensors[1].beacon_is_possible(Vec2::new(9, -2)));
        assert!(sensors[1].beacon_is_possible(Vec2::new(7, -2)));
        assert!(sensors[1].beacon_is_possible(Vec2::new(8, -3)));

    }

}