use crate::input::input::InputParser;
use crate::utility::parser::Parser;

type ItemWorryLevel = u32;
type Destination = usize;


pub fn solve_problem_11a(input: String) -> usize {
    let mut pool = MonkeyPool::from_string(input);
    pool.execute_rounds(20, true);
    let mut inspection_counts = pool.get_inspection_counts();
    inspection_counts.sort();
    let inspection_counts = inspection_counts.into_iter().rev().collect::<Vec<_>>();
    inspection_counts[0] * inspection_counts[1]
}

fn solve_problem_11b(input: String) -> usize {
    let mut pool = MonkeyPool::from_string(input);
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

    pub fn from_string(s: String) -> Self {
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


        Self::new(monkeys)
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
    items: Vec<u32>,
    operation: Operation,
    test_divisor: u32,
    true_destination: usize,
    false_destination: usize,
    inspection_count: usize,
}

impl Monkey {

    pub fn new(
        items: Vec<u32>,
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

    pub fn get_items(&self) -> Vec<u32> {
        self.items.clone()
    }

    pub fn apply_operation(&self, n: u32) -> u32 {
        self.operation.apply(n)
    }

    pub fn test(&self, n: u32) -> bool {
        n % self.test_divisor == 0
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
        
        let maybe_apply_relief = |n: ItemWorryLevel| if with_relief { Self::apply_relief(n) } else { n };
        let to_return: Vec<(ItemWorryLevel, Destination)> = self.items.iter().map(
            |n| maybe_apply_relief(self.apply_operation(*n))
        ).map(
            |new_worry_level| (new_worry_level, self.get_destination(self.test(new_worry_level)))
        ).collect();

        (0..to_return.len()).into_iter().for_each(|_| self.inspection_count += 1);

        self.items.clear();

        return to_return;
    }

    pub fn apply_relief(n: ItemWorryLevel) -> ItemWorryLevel {
        n / 3
    }

    pub fn push(&mut self, n: ItemWorryLevel) {
        self.items.push(n);
    }
    
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

impl Operation {

    pub fn apply(&self, n: u32) -> u32 {
        match self {
            Self::Add(m) => {n + m},
            Self::Multiply(m) => {n * m},
            Self::Square => { n * n }
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
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_parses_into_monkey_pool() {
        let pool = MonkeyPool::from_string(get_example_monkey_config());
        assert_eq!(pool.n_monkeys(), 4);

        let monkeys = pool.get_monkeys();
        assert_eq!(monkeys[0].get_items(), vec![79, 98]);
        assert_eq!(monkeys[3].get_items(), vec![74]);

        assert_eq!(monkeys[1].apply_operation(5), 11);
        assert_eq!(monkeys[2].test(26), true);
        assert_eq!(monkeys[3].test(4), false);

        assert_eq!(monkeys[2].get_destination(true), 1);
        assert_eq!(monkeys[2].get_destination(false), 3);
    }

    #[test]
    fn test_monkey_pool_executes_round() {
        let mut pool = MonkeyPool::from_string(get_example_monkey_config());
        assert_eq!(pool.n_monkeys(), 4);

        pool.execute_round(true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items(), vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].get_items(), vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

        pool.execute_round(true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items(), vec![695, 10, 71, 135, 350]);
        assert_eq!(monkeys[1].get_items(), vec![43, 49, 58, 55, 362]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

        pool.execute_round(true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items(), vec![16, 18, 21, 20, 122]);
        assert_eq!(monkeys[1].get_items(), vec![1468, 22, 150, 286, 739]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

        pool.execute_rounds(17, true);
        let monkeys = pool.get_monkeys().clone();
        assert_eq!(monkeys[0].get_items(), vec![10, 12, 14, 26, 34]);
        assert_eq!(monkeys[1].get_items(), vec![245, 93, 53, 199, 115]);
        assert_eq!(monkeys[2].get_items().len(), 0);
        assert_eq!(monkeys[3].get_items().len(), 0);

    }


}