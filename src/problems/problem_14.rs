use std::collections::{HashMap, HashSet};

use crate::input::input::InputParser;
use crate::utility::vector::Vec2;

pub fn solve_problem_14a(input: Vec<String>) -> usize {
    let mut cave = Cave::from_strings(input, None);
    let sand_entry_location = Vec2::new(500, 0);
    
    cave.drop_sand_until_abyss(&sand_entry_location);

    cave.count_non_abyss_sand()
}

fn solve_problem_14b(input: Vec<String>) -> usize {
    let mut cave = Cave::from_strings(input, Some(2));
    let sand_entry_location = Vec2::new(500, 0);
    
    cave.drop_sand_until_abyss(&sand_entry_location);
    assert!(!cave.has_hit_abyss);

    cave.count_non_abyss_sand()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cave {
    material_map: HashMap<Vec2, Material>,
    lowest_points: HashMap<i32, i32>,
    floor_y: Option<i32>,
    has_hit_abyss: bool,
    has_blocked_starting_point: bool,
}

impl Cave {

    pub fn new(material_map: HashMap<Vec2, Material>, lowest_points: HashMap<i32, i32>, floor_y: Option<i32>) -> Self {
        Self { material_map, lowest_points, floor_y, has_hit_abyss: false, has_blocked_starting_point: false }
    }

    pub fn from_strings(strings: Vec<String>, floor_delta_y: Option<i32>) -> Self {
        let rock_points: HashSet<Vec2> = strings.into_iter().map(|s| Self::lines_to_points(s)).flatten().collect();
        let lowest_points = rock_points.clone().into_iter().map(|p| (p.x(), p.y())).fold(HashMap::new(), |mut accumulator, element| {
            if accumulator.contains_key(&element.0) {
                if &element.1 > accumulator.get(&element.0).unwrap() { // Note that > corresponds to lower!
                    accumulator.insert(element.0, element.1);
                }
            } else {
                accumulator.insert(element.0, element.1);
            }
            accumulator
        });
        let lowest_point = lowest_points.iter().map(|p| *p.1).max().expect("We should always have at least one lowest point.");
        let floor_y = floor_delta_y.map(|delta_y| lowest_point + delta_y);
        let material_map = rock_points.into_iter().map(|p| (p, Material::Rock)).collect();
        Self::new(material_map, lowest_points, floor_y)
    }

    fn lines_to_points(lines: String) -> HashSet<Vec2> {
        let endpoints: Vec<_> = lines
            .split(" -> ")
            .map(|s| s.split(",").collect::<Vec<_>>())
            .map(|coords| {
                assert_eq!(coords.len(), 2);
                Vec2::new(
                    coords[0].parse::<i32>().expect("x should parse to i32"),
                    coords[1].parse::<i32>().expect("y should parse to i32")
                )
            }).collect();

        endpoints.clone().into_iter().zip(endpoints.into_iter().skip(1))
            .map(|(p, q)| Vec2::get_points_between(p, q))
            .flatten().collect::<HashSet<Vec2>>()
    }

    pub fn get(&self, v: &Vec2) -> Material {
        match self.material_map.get(v) {
            None => Material::Air,
            Some(m) => *m,
        }
    }

    pub fn drop_sand_until_abyss(&mut self, starting_position: &Vec2) {
        while !(self.has_hit_abyss || self.has_blocked_starting_point) {
            self.drop_sand(starting_position);
        }
    }

    pub fn drop_sand(&mut self, starting_position: &Vec2) {

        if self.get(starting_position) == Material::Sand {
            self.has_blocked_starting_point = true;
            return;
        }

        let mut sand_position = starting_position.clone();
        loop {
            match self.get_next_sand_position(sand_position) {
                Some(v) => {
                    if sand_position == v {
                        self.material_map.insert(sand_position, Material::Sand);
                        break;
                    }
                    sand_position = v;
                },
                None => {
                    self.has_hit_abyss = true;
                    break;
                }
            }
        }
    }

    fn get_next_sand_position(&self, v: Vec2) -> Option<Vec2> {
        let below = v + Vec2::j();
        let below_left = below - Vec2::i();
        let below_right = below + Vec2::i();
        if self.is_in_abyss(&v) {
            return None;
        }
        match self.floor_y {
            None => {},
            Some(y) => {
                if y - 1 == v.y() { // y - 1 is the space above the floor
                    return Some(v);
                }
            }
        }
        let below = v + Vec2::j();
        let below_left = below - Vec2::i();
        let below_right = below + Vec2::i();
        if self.get(&below) == Material::Air {
            return Some(below);
        }
        if self.get(&below_left) == Material::Air {
            return Some(below_left);
        }
        if self.get(&below_right) == Material::Air {
            return Some(below_right);
        }
        return Some(v);
    }

    fn is_in_abyss(&self, v: &Vec2) -> bool {
        match self.floor_y {
            None => {},
            Some(y) => {return v.y() > y;}
        }
        match self.lowest_points.get(&v.x()) {
            None => true,
            Some(lowest_rock_y) => &v.y() > lowest_rock_y // > corresponds to lower!
        }
    }

    fn has_sand_in_abyss(&self) -> bool {
        self.has_hit_abyss
    }

    pub fn count_non_abyss_sand(&self) -> usize {
        self.material_map.values().filter(|m| m == &&Material::Sand).count()
    }

}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[cfg(test)]
mod test_problem_14 {

    use super::*;

    fn get_example_input() -> Vec<String> {
        vec![
            "498,4 -> 498,6 -> 496,6".to_string(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
        ]
    }

    #[test]
    fn test_problem_14a_passes() {
        
        assert_eq!(solve_problem_14a(get_example_input()), 24);

        let input = InputParser::new().parse_as_string("input_14.txt").unwrap();

        let answer = solve_problem_14a(input);
        assert_eq!(answer, 763);
    }
    
    #[test]
    fn test_problem_14b_passes() {

        assert_eq!(solve_problem_14b(get_example_input()), 93);

        let input = InputParser::new().parse_as_string("input_14.txt").unwrap();

        let answer = solve_problem_14b(input);
        assert_eq!(answer, 23921);
    }

    #[test]
    fn test_simulates_sand_drops() {
        
        let mut cave = Cave::from_strings(get_example_input(), None);
        let sand_entry_location = Vec2::new(500, 0);
        cave.drop_sand(&sand_entry_location);

        assert!(!cave.has_sand_in_abyss());
        assert_eq!(cave.get(&Vec2::new(500, 8)), Material::Sand);
        assert_eq!(cave.get(&Vec2::new(500, 9)), Material::Rock);
        assert_eq!(cave.get(&Vec2::new(499, 8)), Material::Air);
        assert_eq!(cave.get(&Vec2::new(501, 8)), Material::Air);

        cave.drop_sand(&sand_entry_location);
        assert!(!cave.has_sand_in_abyss());
        assert_eq!(cave.get(&Vec2::new(500, 8)), Material::Sand);
        assert_eq!(cave.get(&Vec2::new(500, 9)), Material::Rock);
        assert_eq!(cave.get(&Vec2::new(499, 8)), Material::Sand);
        assert_eq!(cave.get(&Vec2::new(501, 8)), Material::Air);

        cave.drop_sand_until_abyss(&sand_entry_location);
        assert!(cave.has_sand_in_abyss());
        assert_eq!(cave.count_non_abyss_sand(), 24);

    }

    #[test]
    fn test_simulates_sand_drops_with_floor() {
        
        let mut cave = Cave::from_strings(get_example_input(), Some(2));
        let sand_entry_location = Vec2::new(500, 0);
        cave.drop_sand(&sand_entry_location);

        assert!(!cave.has_sand_in_abyss());
        assert_eq!(cave.get(&Vec2::new(500, 8)), Material::Sand);
        assert_eq!(cave.get(&Vec2::new(500, 9)), Material::Rock);
        assert_eq!(cave.get(&Vec2::new(499, 8)), Material::Air);
        assert_eq!(cave.get(&Vec2::new(501, 8)), Material::Air);

        cave.drop_sand(&sand_entry_location);
        assert!(!cave.has_sand_in_abyss());
        assert_eq!(cave.get(&Vec2::new(500, 8)), Material::Sand);
        assert_eq!(cave.get(&Vec2::new(500, 9)), Material::Rock);
        assert_eq!(cave.get(&Vec2::new(499, 8)), Material::Sand);
        assert_eq!(cave.get(&Vec2::new(501, 8)), Material::Air);

    }

    #[test]
    fn test_computes_abyss_correctly() {
        
        let cave = Cave::from_strings(get_example_input(), None);

        assert!(cave.is_in_abyss(&Vec2::new(493, 0)));
        assert!(cave.is_in_abyss(&Vec2::new(504, 0)));
        assert!(cave.is_in_abyss(&Vec2::new(500, 10)));
        assert!(cave.is_in_abyss(&Vec2::new(503, 5)));

        assert!(!cave.is_in_abyss(&Vec2::new(500, 0)));
        assert!(!cave.is_in_abyss(&Vec2::new(499, 4)));
        assert!(!cave.is_in_abyss(&Vec2::new(500, 9)));
        assert!(!cave.is_in_abyss(&Vec2::new(503, 4)));
    }

}