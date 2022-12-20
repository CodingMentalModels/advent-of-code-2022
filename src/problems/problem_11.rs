use std::ops::{Add, Mul};

use crate::input::input::InputParser;
use crate::utility::parser::Parser;

type Destination = usize;


pub fn solve_problem_11a(input: String) -> usize {
    let mut pool = MonkeyPool::from_string(input, false);
    pool.execute_rounds(20, true);
    let mut inspection_counts = pool.get_inspection_counts();
    inspection_counts.sort();
    let inspection_counts = inspection_counts.into_iter().rev().collect::<Vec<_>>();
    inspection_counts[0] * inspection_counts[1]
}

fn solve_problem_11b(input: String) -> usize {
    let mut pool = MonkeyPool::from_string(input, true);
    pool.execute_rounds(10000, false);
    let mut inspection_counts = pool.get_inspection_counts();
    inspection_counts.sort();
    let inspection_counts = inspection_counts.into_iter().rev().collect::<Vec<_>>();
    inspection_counts[0] * inspection_counts[1]
}

#[derive(Clone, Debug)]
struct MonkeyPool {
    monkeys: Vec<Monkey>,
}

impl MonkeyPool {

    pub fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }

    pub fn from_string(s: String, use_moduli: bool) -> Self {
        let mut parser = Parser::new(s);
        let mut monkeys = Vec::new();
        while let Ok(_monkey_header) = parser.expect("Monkey ") {
            let _monkey_number = parser.consume_line();

            let starting_items = Self::consume_starting_items(&mut parser);
            let operation = Self::consume_operation(&mut parser);
            let (test_divisor, true_destination, false_destination) = Self::consume_test(&mut parser);

            let monkey = Monkey::new(starting_items, operation, test_divisor, true_destination, false_destination);
            monkeys.push(monkey);
            let _whitespace = parser.consume_whitespace();
        }

        parser.expect_end_of_line_or_file().expect("Should have hit the end of the file.");


        let pool = Self::new(monkeys);
        if use_moduli {
            pool.modulize()
        } else {
            pool
        }

    }

    pub fn modulize(self) -> Self {
        let moduli = self.monkeys.iter().map(|m| m.get_test_divisor()).collect::<Vec<_>>();
        Self::new(self.monkeys.into_iter().map(|m| m.modulize(&moduli)).collect())
    }

    pub fn n_monkeys(&self) -> usize {
        self.monkeys.len()
    }

    pub fn get_monkeys(&self) -> &Vec<Monkey> {
        &self.monkeys
    }

    fn consume_starting_items(parser: &mut Parser) -> Vec<ItemWorryLevel> {
            let _whitespace = parser.consume_whitespace();
            let _starting_items_header = parser.expect("Starting items: ").unwrap();
            let starting_items_string = parser.consume_line();
            
            let starting_items = starting_items_string
                .split(", ")
                .map(|s| s.trim().parse::<u32>().expect(&format!("Should be able to parse item number: {}", s)))
                .map(|n| ItemWorryLevel::N(n))
                .collect::<Vec<_>>();
            return starting_items;
    }

    fn consume_operation(parser: &mut Parser) -> Operation {
        let _whitespace = parser.consume_whitespace();
        let _operation_header = parser.expect("Operation: new = old ").unwrap();
        let operand = parser.consume_n(1);
        let _whitespace = parser.consume_whitespace();
        let n_string = parser.consume_until_whitespace();
        if n_string.trim() == "old" {
            let to_return = Operation::Square;
            parser.expect_end_of_line().expect("We should have hit the end of a line.");
            return to_return;
        }
        let n = n_string.trim().parse::<u32>().expect(&format!("n should be parsable into u32: {}", n_string));

        let to_return = match operand.as_str() {
            "+" => Operation::Add(n),
            "*" => Operation::Multiply(n),
            _ => panic!("Invalid operand: {}", operand)
        };

        parser.expect_end_of_line().expect("We should have hit the end of a line.");

        return to_return;
    }

    fn consume_test(parser: &mut Parser) -> (u32, Destination, Destination) {
        let _whitespace = parser.consume_whitespace();
        let _test_header = parser.expect("Test: divisible by ").unwrap();
        let test_divisor = parser.consume_until_whitespace().trim().parse::<u32>().expect("A valid number to divide by.");
        let _whitespace = parser.consume_whitespace();
        let true_destination = Self::consume_destination(parser, true);
        let _whitespace = parser.consume_whitespace();
        let false_destination = Self::consume_destination(parser, false);

        (test_divisor, true_destination, false_destination)
    }

    fn consume_destination(parser: &mut Parser, condition: bool) -> Destination {
        let condition_text = if condition { "true" } else { "false" };
        let _header = parser.expect(&format!("If {}: throw to monkey ", condition_text));
        parser.consume_until_whitespace().trim().parse::<usize>().expect("A valid monkey destination.")
    }

    pub fn execute_rounds(&mut self, n: usize, with_relief: bool) {
        (0..n).into_iter().for_each(|_| self.execute_round(with_relief))
    }

    pub fn execute_round(&mut self, with_relief: bool) {
        for i in (0..self.n_monkeys()) {
            for (item, destination) in self.monkeys[i].execute_round(with_relief).into_iter() {
                self.monkeys[destination].push(item);
            }
        }
    }

    pub fn get_inspection_counts(&self) -> Vec<usize> {
        self.monkeys.iter().map(|m| m.get_inspection_count()).collect()
    }

}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Monkey {
    items: Vec<ItemWorryLevel>,
    operation: Operation,
    test_divisor: u32,
    true_destination: usize,
    false_destination: usize,
    inspection_count: usize,
}

impl Monkey {

    pub fn new(
        items: Vec<ItemWorryLevel>,
        operation: Operation,
        test_divisor: u32,
        true_destination: usize,
        false_destination: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test_divisor,
            true_destination,
            false_destination,
            inspection_count: 0,
        }
    }

    pub fn modulize(self, moduli: &Vec<u32>) -> Self {
        Self::new(
            self.items.into_iter().map(|i| i.modulize(moduli)).collect(),
            self.operation,
            self.test_divisor,
            self.true_destination,
            self.false_destination
        )
    }
    pub fn get_items(&self) -> Vec<ItemWorryLevel> {
        self.items.clone()
    }

    pub fn apply_operation(&self, n: ItemWorryLevel) -> ItemWorryLevel {
        self.operation.apply(n)
    }

    pub fn get_test_divisor(&self) -> u32 {
        self.test_divisor
    }

    pub fn test(&self, n: &ItemWorryLevel) -> bool {
        n.is_disible_by(self.test_divisor)
    }

    pub fn get_destination(&self, b: bool) -> Destination {
        if b {
            self.true_destination
        } else {
            self.false_destination
        }
    }

    pub fn get_inspection_count(&self) -> usize {
        self.inspection_count
    }

    pub fn execute_round(&mut self, with_relief: bool) -> Vec<(ItemWorryLevel, Destination)> {
        
        let maybe_apply_relief = |n: ItemWorryLevel| if with_relief {
                match n {
                    ItemWorryLevel::N(n) => ItemWorryLevel::N(Self::apply_relief(n)),
                    ItemWorryLevel::Moduli(_) => panic!("Can't apply relief using moduli.")
                }
            } else { n };
        let to_return: Vec<(ItemWorryLevel, Destination)> = self.items.iter().map(
            |n| maybe_apply_relief(self.apply_operation(n.clone()))
        ).map(
            |new_worry_level| (new_worry_level.clone(), self.get_destination(self.test(&new_worry_level)))
        ).collect();

        (0..to_return.len()).into_iter().for_each(|_| self.inspection_count += 1);

        self.items.clear();

        return to_return;
    }

    pub fn apply_relief(n: u32) -> u32 {
        n / 3
    }

    pub fn push(&mut self, n: ItemWorryLevel) {
        self.items.push(n);
    }
    
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum ItemWorryLevel {
    N(u32),
    Moduli(ItemWorryLevelModuli),
}

impl Add<u32> for ItemWorryLevel {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        match self {
            Self::N(n) => Self::N(n + other),
            Self::Moduli(moduli) => {
                let new_levels = moduli.moduli.iter().zip(moduli.levels).map(|(modulus, level)| (level + other) % modulus).collect();
                Self::Moduli(ItemWorryLevelModuli::new(new_levels, moduli.moduli))
            }
        }
    }
}

impl Mul<u32> for ItemWorryLevel {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        match self {
            Self::N(n) => Self::N(n * other),
            Self::Moduli(moduli) => {
                let new_levels = moduli.moduli.iter().zip(moduli.levels).map(|(modulus, level)| (level * other) % modulus).collect();
                Self::Moduli(ItemWorryLevelModuli::new(new_levels, moduli.moduli))
            }
        }
    }
}

impl Mul<Self> for ItemWorryLevel {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::N(n), Self::N(m)) => Self::N(n * m),
            (Self::Moduli(moduli_n), Self::Moduli(moduli_m)) => {
                assert_eq!(moduli_n.moduli, moduli_m.moduli);
                let new_levels = moduli_n.moduli.iter().zip(moduli_n.levels).zip(moduli_m.levels).map(|((modulus, n_level), m_level)| (n_level * m_level) % modulus).collect();
                Self::Moduli(ItemWorryLevelModuli::new(new_levels, moduli_n.moduli))
            },
            (_, _) => panic!("Unhandled cases.")
        }
    }
}

impl ItemWorryLevel {

    pub fn modulize(self, moduli: &Vec<u32>) -> Self {
        match self {
            Self::N(n) => { Self::Moduli(ItemWorryLevelModuli::from_n(n, moduli.clone())) },
            Self::Moduli(m) => { panic!("Already modulized!") },
        }
    }

    pub fn unwrap(&self) -> u32 {
        match self {
            Self::N(n) => *n,
            _ => panic!("Can't unwrap a moduli!")
        }
    }

    pub fn from_n(n: u32, maybe_moduli: Option<Vec<u32>>) -> Self {
        match maybe_moduli {
            None => Self::N(n),
            Some(moduli) => Self::Moduli(ItemWorryLevelModuli::from_n(n, moduli))
        }
    }

    pub fn is_disible_by(&self, divisor: u32) -> bool {
        match self {
            Self::N(n) => {
                n % divisor == 0
            },
            Self::Moduli(moduli) => {
                match moduli.moduli.binary_search(&divisor) {
                    Ok(i) => {
                        moduli.levels[i] == 0
                    },
                    Err(e) => panic!("Divisor wasn't found in moduli: {}", divisor)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ItemWorryLevelModuli {
    levels: Vec<u32>,
    moduli: Vec<u32>,
}

impl ItemWorryLevelModuli {

    pub fn new(levels: Vec<u32>, moduli: Vec<u32>) -> Self {
        assert_eq!(levels.len(), moduli.len());
        let mut pairs = levels.into_iter().zip(moduli).collect::<Vec<(u32, u32)>>();
        pairs.sort_by(|a, b| a.1.cmp(&b.1));
        let levels = pairs.iter().map(|x| x.0).collect();
        let moduli = pairs.iter().map(|x| x.1).collect();
        Self { levels, moduli }
    }

    pub fn from_n(n: u32, moduli: Vec<u32>) -> Self {
        let levels = moduli.iter().map(|m| n % m).collect();
        Self::new(levels, moduli)
    }

}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

impl Operation {

    pub fn apply(&self, n: ItemWorryLevel) -> ItemWorryLevel {
        match self {
            Self::Add(m) => {n + *m},
            Self::Multiply(m) => {n * *m},
            Self::Square => { n.clone() * n }
        }
    }
}

#[cfg(test)]
mod test_problem_11 {

    use super::*;

    fn get_example_monkey_config() -> String {
        InputParser::new().parse_to_single_string("example_input_11.txt").unwrap()
    }

    #[test]
    fn test_problem_11a_passes() {
        
        assert_eq!(solve_problem_11a(get_example_monkey_config()), 10605);

        let input = InputParser::new().parse_to_single_string("input_11.txt").unwrap();

        let answer = solve_problem_11a(input);
        assert_eq!(answer, 64032);
    }
    
    #[test]
    fn test_problem_11b_passes() {
        assert_eq!(solve_problem_11b(get_example_monkey_config()), 2713310158);

        let input = InputParser::new().parse_to_single_string("input_11.txt").unwrap();

        let answer = solve_problem_11b(input);
        assert_eq!(answer, 12729522272);
    }

    #[test]
    fn test_parses_into_monkey_pool() {
        let pool = MonkeyPool::from_string(get_example_monkey_config(), false);
        assert_eq!(pool.n_monkeys(), 4);

        let monkeys = pool.get_monkeys();
        assert_eq!(monkeys[0].get_items(), vec![ItemWorryLevel::N(79), ItemWorryLevel::N(98)]);
        assert_eq!(monkeys[3].get_items(), vec![ItemWorryLevel::N(74)]);

        assert_eq!(monkeys[1].apply_operation(ItemWorryLevel::N(5)), ItemWorryLevel::N(11));
        assert_eq!(monkeys[2].test(&ItemWorryLevel::N(26)), true);
        assert_eq!(monkeys[3].test(&ItemWorryLevel::N(4)), false);

        assert_eq!(monkeys[2].get_destination(true), 1);
        assert_eq!(monkeys[2].get_destination(false), 3);
    }

    #[test]
    fn test_monkey_pool_executes_round() {
        let mut pool = MonkeyPool::from_string(get_example_monkey_config(), false);
        assert_eq!(pool.n_monkeys(), 4);

        pool.execute_round(true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

        pool.execute_round(true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![695, 10, 71, 135, 350]);
        assert_eq!(monkeys[1].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![43, 49, 58, 55, 362]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

        pool.execute_round(true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![16, 18, 21, 20, 122]);
        assert_eq!(monkeys[1].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![1468, 22, 150, 286, 739]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

        pool.execute_rounds(17, true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].get_items().into_iter().map(|i| i.unwrap()).collect::<Vec<u32>>(), vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

    }


    #[test]
    fn test_monkey_pool_executes_round_with_moduli() {
        let mut pool = MonkeyPool::from_string(get_example_monkey_config(), true);
        assert_eq!(pool.n_monkeys(), 4);

        pool.execute_round(false);
        assert_eq!(pool.get_inspection_counts(), vec![2, 4, 3, 6]);

        pool.execute_rounds(19, false);
        assert_eq!(pool.get_inspection_counts(), vec![99, 97, 8, 103]);

    }

    #[test]
    fn test_monkey_pool_executes_round_equivalently_using_moduli_or_not() {
        let mut pool_without = MonkeyPool::from_string(get_example_monkey_config(), false);
        let mut pool_with = pool_without.clone().modulize();

        assert_eq!(pool_without.n_monkeys(), pool_with.n_monkeys());

        for i in 0..4 {
            pool_without.execute_round(false);
            pool_with.execute_round(false);

            let monkeys_without = pool_without.get_monkeys().clone();
            let monkeys_with = pool_with.get_monkeys().clone();

            for (monkey_without, monkey_with) in monkeys_without.into_iter().zip(monkeys_with) {
                assert_eq!(monkey_without.operation, monkey_with.operation);
                assert_eq!(monkey_without.test_divisor, monkey_with.test_divisor);
                assert_eq!(monkey_without.true_destination, monkey_with.true_destination);
                assert_eq!(monkey_without.false_destination, monkey_with.false_destination);
                assert_eq!(monkey_without.items.len(), monkey_with.items.len());
            }
            assert_eq!(pool_without.get_inspection_counts(), pool_with.get_inspection_counts(), "Failed at {}", i);
        }
    }

    #[test]
    fn test_monkey_pool_modulizes() {
        let mut pool = MonkeyPool::from_string(get_example_monkey_config(), true);

        assert_eq!(pool.n_monkeys(), 4);

        pool.execute_round(false);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);
    }


}