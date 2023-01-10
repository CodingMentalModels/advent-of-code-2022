use std::collections::{HashMap, HashSet};

use crate::input::input::InputParser;

type Time = usize;

const INITIAL_MINUTES_UNTIL_ERUPTION: Time = 30;

pub fn solve_problem_16a(input: Vec<String>) -> u32 {
        let valves = Valve::from_strings(input);
        let graph = ValveGraph::new(valves.clone());
        
        let initial_t = INITIAL_MINUTES_UNTIL_ERUPTION;

        let mut states = graph.get_labels().clone().into_iter()
            .map(|l| {
                let mut cloned = graph.clone();
                cloned.open(&l);
                vec![(graph.clone(), l.clone()), (cloned, l)]
            }).flatten().map(|(g, l)| g.get_state(initial_t, &l)).collect::<HashSet<_>>();

        let mut solutions = states.iter()
            .map(
            |s| graph.get_previous_moves_from_state(*s).into_iter()
                .map(|m| ((s.player_position, s.is_open(s.player_position), m), graph.update_from_state(*s).get_cumulative_pressure_from(initial_t))).collect::<Vec<_>>()
            ).flatten().collect::<HashMap<_, _>>();

        for t in (0..INITIAL_MINUTES_UNTIL_ERUPTION).into_iter().rev() {
            let previous_states = states.iter()
                .map(|s| graph.get_previous_states_from_state(*s))
                .flatten()
                .collect::<HashSet<_>>();
            let best_outcomes = previous_states.iter().map(
                |s| {
                    let g = graph.update_from_state(*s);
                    let l = g.index_to_label(s.player_position);
                    (
                            s,
                            g.get_moves(&l).into_iter()
                                .fold(0, |acc, m| {
                                    let key = (s.player_position, s.is_open(s.player_position), m);
                                    let new_solution = solutions.get(&key).expect(&format!("We should already have calculated {:?}.", key));
                                    if new_solution > &acc {
                                        *new_solution
                                    } else {
                                        acc
                                    }
                                }
                            ) + g.get_flow_rate()
                        )
                }
            ).map(|(s, pressure_released)| 
                graph.get_previous_moves_from_state(*s).into_iter().map(|m| ((s.player_position, s.is_open(s.player_position), m), pressure_released)).collect::<Vec<_>>()
            ).flatten().collect();
            solutions = best_outcomes;
            states = previous_states;
        }

        let graph_state = graph.get_state(0, "AA");
        *graph.get_moves("AA").into_iter()
            .map(|m| solutions.get(&(graph_state.player_position, graph_state.is_open(graph_state.player_position), m)).expect("We should have tested all available moves"))
            .max().expect("There will be at least one move.")

    }


fn solve_problem_16b(input: Vec<String>) -> usize {
    unimplemented!();
}

fn get_minutes_until_eruption(t: usize) -> usize {
    INITIAL_MINUTES_UNTIL_ERUPTION - t
}

fn get_causal_radius(t: usize) -> usize {
    assert!(t < INITIAL_MINUTES_UNTIL_ERUPTION);
    match get_minutes_until_eruption(t) {
        0..=1 => 0,
        n => n - 2,
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Edge(String, Move);

impl Edge {

    pub fn get_position(&self) -> String {
        self.0.clone()
    }

    pub fn get_move(&self) -> Move {
        self.1
    }

    pub fn get_edges_from(graph: ValveGraph, position: &str) -> HashSet<Edge> {
        graph.get_moves(position).into_iter().map(|m| Edge(position.to_string(), m)).collect()
    }

    pub fn get_edges_to(graph: ValveGraph, position: &str) -> HashSet<Edge> {
        graph.get_previous_moves(position).into_iter().map(|m| {
            match m {
                Move::Open => { Edge(position.to_string(), m) },
                Move::Walk(l) => { Edge(l, m) },
            }
        }).collect()
    }
}

#[derive(Clone, Debug)]
struct ConditionalOptima(HashMap<Option<String>, u32>);

impl ConditionalOptima {

    pub fn new(labels: Vec<String>) -> Self {
        let mut to_return = labels.into_iter().map(|l| (Some(l), 0)).collect::<HashMap<_, _>>();
        to_return.insert(None, 0);
        Self(to_return)
    }

    pub fn get_optimum(&self) -> u32 {
        *self.0.get(&None).expect("Was defined on construction")
    }

    pub fn get_optimum_if_open(&self, label: &str) -> u32 {
        *self.0.get(&Some(label.to_string())).expect("Should be a valid label")
    }
}

#[derive(Clone, Debug)]
struct Solver {
    graph: ValveGraph,
    solutions: HashMap<Time, HashMap<Edge, IsOpenOptima>>,
}

impl Solver {

    pub fn new(graph: ValveGraph) -> Self {
        Self {graph, solutions: HashMap::new()}
    }
    
    pub fn solve(&mut self, time_until_eruption: Time, starting_position: &str) -> u32 {

        self.solutions = HashMap::new();
        
        let mut final_solutions = self.graph.get_labels().clone().into_iter()
            .map(|l| {
                Edge::get_edges_to(self.graph, &l)
            }).flatten().map(|e| (e, ConditionalOptima::new(self.graph.get_labels()))).collect::<HashMap<_, _>>();
        
        self.solutions.insert(time_until_eruption, final_solutions);

        for t in (0..time_until_eruption).into_iter().rev() {
            let solutions_t = self.solutions.get(t).expect("Should have been defined on a previous loop");
            let mut solutions_t_minus_1 = HashMap::new();
            for (edge, optima) in solutions_t.iter() {
                let (position, m) = edge.unpack();
                let new_edges = Edge::get_edges_from(self.graph, position);
                let mut new_optima = optima.clone();
                
            }
        }
    }

    pub fn get_best_evaluation(&self, t: Time, starting_position: &str) -> u32 {
        *self.solutions.get(&t).unwrap()
            .iter()
            .filter(|(e, eval)| &e.1 == starting_position)
            .map(|(e, eval)| eval)
            .max().unwrap()
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Move {
    Open,
    Walk(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ValveGraph {
    ordering: HashMap<String, usize>,
    valves: Vec<Valve>,
}

impl ValveGraph {

    pub fn new(valves: Vec<Valve>) -> Self {
        let labels = valves.iter().enumerate().map(|(i, v)| (v.get_label(), i)).collect();
        ValveGraph { ordering: labels, valves: valves}
    }

    pub fn get_valve(&self, label: &str) -> Option<Valve> {
        self.ordering.get(label).map(|i| self.valves[*i].clone())
    }

    pub fn index_to_label(&self, i: usize) -> String {
        self.valves[i].get_label()
    }

    pub fn get_labels(&self) -> Vec<String> {
        self.valves.iter().map(|v| v.get_label()).collect()
    }
    
    pub fn get_previous_graphs(&self, player_position: &str) -> Vec<(ValveGraph, String)> {
        let moves = self.get_moves(player_position);

        let mut from_graphs = moves.into_iter().filter(|m| m != &Move::Open).map(|m| {
            match m {
                Move::Open => panic!(),
                Move::Walk(l) => (self.clone(), l)
            }
        }).map(|(g, l)| {
            let mut cloned = g.clone();
            if self.is_open(&l).unwrap() {
                vec![(g, l)]
            } else {
                cloned.open(&l);
                vec![(g, l.clone()), (cloned, l)]
            }
        }).flatten().collect::<Vec<_>>();

        if self.is_closed(player_position).expect("Player position should be a valid label") {
            let mut cloned = self.clone();
            cloned.close(player_position);
            from_graphs.push((cloned, player_position.to_string()));
        }

        return from_graphs;

    }

    pub fn get_previous_moves(&self, player_position: &str) -> Vec<Move> {
        if self.is_closed(player_position).unwrap() {
            vec![
                Move::Open,
                Move::Walk(player_position.to_string()),
            ]
        } else {
            vec![
                Move::Walk(player_position.to_string()),
            ]
        }
    }

    pub fn get_previous_states_from_state(&self, state: ValveGraphState) -> Vec<ValveGraphState> {
        self.update_from_state(state)
            .get_previous_graphs(&self.valves[state.player_position].get_label())
            .into_iter().map(|(g, l)| g.get_state(state.t - 1, &l)).collect()
    }
    
    pub fn update_from_state(&self, state: ValveGraphState) -> Self {
        let mut to_return = self.clone();
        to_return.close_all();

        let mut valve_states = state.valve_states;
        let mut i = 0;
        while valve_states > 0 {
            if valve_states % 2 == 1 {
                to_return.open(&to_return.valves[i].get_label());
            }
            valve_states = valve_states >> 1;
            i += 1;
        }
        return to_return;
    }

    pub fn get_previous_moves_from_state(&self, s: ValveGraphState) -> Vec<Move> {
        self.update_from_state(s).get_previous_moves(&self.valves[s.player_position].get_label())
    }

    pub fn get_state_after_move(&self, t: usize, player_position: &str, m: Move) -> ValveGraphState {
        assert!(self.get_moves(player_position).contains(&m));
        let (new_graph, new_position) = self.make_move(player_position, m);
        new_graph.get_state(t + 1, &new_position)
    }

    pub fn get_state(&self, t: usize, player_position: &str) -> ValveGraphState {
        let player_index = self.ordering.get(player_position).expect("Player position should be a valid label");
        let valve_states = self.get_open_indices().iter().fold(0_u32, |acc, elt| acc + (1 << (*elt as u32)));
        ValveGraphState::new(t, *player_index, valve_states)
    }

    pub fn make_move(&self, player_position: &str, m: Move) -> (Self, String) {
        assert!(self.get_moves(player_position).contains(&m));
        let mut cloned = self.clone();
        match m {
            Move::Open => {
                cloned.open(player_position);
                return (cloned, player_position.to_string());
            },
            Move::Walk(l) => {
                return (cloned, l);
            }
        }
    }

    pub fn get_moves(&self, current_position: &str) -> Vec<Move> {
        let mut to_return = if self.is_closed(current_position).expect("Current position should be a valid valve label.") {
            vec![Move::Open]
        } else {
            Vec::new()
        };

        to_return.append(&mut self.get_neighbors(current_position).unwrap().iter().map(|l| Move::Walk(l.clone())).collect());

        return to_return;
    }

    pub fn get_neighbors(&self, label: &str) -> Option<Vec<String>> {
        self.get_valve(label)
            .map(
                |v| v.get_neighbors().clone()
            )
    }

    pub fn len(&self) -> usize {
        self.valves.len()
    }

    pub fn get_open_indices_within(&self, distance: usize, player_position: &str) -> HashSet<usize> {
        let indicies_within = self.get_indices_within(distance, player_position);
        self.get_open_indices().into_iter().filter(|i| indicies_within.contains(&i)).collect()
    }

    pub fn get_open_indices(&self) -> HashSet<usize> {
        self.ordering.iter().filter(|(l, _i)| self.is_open(l).unwrap()).map(|(_l, i)| *i).collect()
    }

    pub fn get_indices_within(&self, distance: usize, player_position: &str) -> HashSet<usize> {
        let mut distance_remaining = distance;
        let mut previous_neighbors = vec![player_position.to_string()].into_iter().collect::<HashSet<_>>();
        let mut to_return = previous_neighbors.clone();
        loop {
            if distance_remaining == 0 {
                break;
            }
            let mut neighbors = HashSet::new();
            for label in previous_neighbors.iter() {
                neighbors = neighbors.union(&self.get_neighbors(&label).unwrap().clone().into_iter().collect::<HashSet<_>>()).cloned().collect();
            }
            to_return = to_return.union(&neighbors).cloned().collect();
            previous_neighbors = neighbors;
            distance_remaining -= 1;
        }
        return to_return.into_iter().map(|l| *self.ordering.get(&l).unwrap()).collect();
    }

    pub fn is_open(&self, label: &str) -> Option<bool> {
        self.get_valve(label).map(|v| v.is_open())
    }

    pub fn is_closed(&self, label: &str) -> Option<bool> {
        self.is_open(label).map(|b| !b)
    }

    pub fn open_all(&mut self) {
        self.valves.iter_mut().for_each(|v| v.open())
    }

    pub fn open_valves(&mut self, labels: Vec<&str>) -> Result<(), String> {
        let results: Result<Vec<()>, String> = labels.into_iter().map(|l| self.open(l).ok_or(format!("No valve with label {}", l))).collect();
        results.map(|_| ())
    }

    pub fn open(&mut self, label: &str) -> Option<()> {
        match self.ordering.get(label) {
            Some(i) => {
                let v = self.valves.get_mut(*i).unwrap();
                v.open();
                return Some(());
            },
            None => {return None;}
        }
    }

    pub fn close_all(&mut self) {
        self.valves.iter_mut().for_each(|v| v.close())
    }

    pub fn close(&mut self, label: &str) -> Option<()> {
        match self.ordering.get(label) {
            Some(i) => {
                let v = self.valves.get_mut(*i).unwrap();
                v.close();
                return Some(());
            },
            None => {return None;}
        }
    }

    pub fn get_flow_rate(&self) -> u32 {
        self.valves.iter().map(|v| if v.is_open() {v.get_flow_rate()} else { 0 }).sum()
    }

    pub fn get_cumulative_pressure_from(&self, t: usize) -> u32 {
        self.valves.iter().map(|v| v.get_cumulative_pressure_from(t)).sum()
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct ValveGraphState {
    t: usize,
    player_position: usize,
    valve_states: u32,
}

impl ValveGraphState {

    pub fn new(t: usize, player_position: usize, valve_states: u32) -> Self {
        Self {t, player_position, valve_states}
    }

    pub fn get_all_permutations(&self, variable_bits: Vec<usize>) -> HashSet<Self> {
        let mut to_return: HashSet<_> = vec![*self].into_iter().collect();
        for i in variable_bits {
            to_return = to_return.into_iter().map(|s: Self| vec![s, s.flip(i)]).flatten().collect()
        }

        return to_return;
    }

    pub fn flip(&self, i: usize) -> Self {
        Self::new(self.t, self.player_position, self.valve_states ^ (1 << i))
    }

    pub fn is_open(&self, i: usize) -> bool {
        (self.valve_states << i) % 2 == 1
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Valve {
    label: String,
    flow_rate: u32,
    neighbors: Vec<String>,
    is_open: bool,
}

impl Valve {

    pub fn new(label: String, flow_rate: u32, neighbors: Vec<String>) -> Self {
        Self {label, flow_rate, neighbors, is_open: false}
    }

    pub fn from_strings(strings: Vec<String>) -> Vec<Self> {
        strings.into_iter().map(Self::from_string).collect()
    }

    pub fn from_string(s: String) -> Self {
        let halves = s.split(';').collect::<Vec<_>>();
        assert_eq!(halves.len(), 2);

        let (flow_rate_half, neighbor_half) = (halves[0], halves[1]);
        let label_and_flow_rate = flow_rate_half.split('=').collect::<Vec<_>>();
        assert_eq!(label_and_flow_rate.len(), 2);

        let flow_rate = label_and_flow_rate[1].parse::<u32>().unwrap();
        let label_parts = label_and_flow_rate[0].split_whitespace().collect::<Vec<_>>();
        assert_eq!(label_parts.len(), 5);
        let label = label_parts[1];

        let mut neighbors_string = &neighbor_half[" tunnels lead to valve".len()..];
        if neighbors_string.starts_with('s') {
            neighbors_string = &neighbors_string[2..];
        } else {
            neighbors_string = &neighbors_string[1..];
        }
        let neighbors = neighbors_string.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

        Self::new(label.to_string(), flow_rate, neighbors)
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }

    pub fn get_flow_rate(&self) -> u32 {
        self.flow_rate
    }

    pub fn get_neighbors(&self) -> &Vec<String> {
        &self.neighbors
    }

    pub fn get_cumulative_pressure_from(&self, t: usize) -> u32 {
        if self.is_open {
            (INITIAL_MINUTES_UNTIL_ERUPTION - t) as u32 * self.flow_rate
        } else { 0 }
    }

    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open()
    }

}

#[cfg(test)]
mod test_problem_16 {

    use super::*;

    fn get_example_input() -> Vec<String> {
        InputParser::new().parse_as_string("example_input_16.txt").unwrap()
    }

    #[test]
    fn test_problem_16a_passes() {
        
        assert_eq!(solve_problem_16a(get_example_input()), 1651);
        
        // let input = InputParser::new().parse_as_string("input_16.txt").unwrap();

        // let answer = solve_problem_16a(input);
        // assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_16b_passes() {
        let input = InputParser::new().parse_as_string("input_16.txt").unwrap();

        let answer = solve_problem_16b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_solver_solves() {
        
        let valves = Valve::from_strings(get_example_input());
        let mut graph = ValveGraph::new(valves.clone());
        let mut solver = Solver::new(graph);
        
        let solution = solver.solve(0, "AA");
        assert_eq!(solution, 0);
        
        let solution = solver.solve(2, "AA");
        assert_eq!(solution, 0);

        let solution = solver.solve(2, "JJ");
        assert_eq!(solution, 21);

        let solution = solver.solve(3, "AA");
        assert_eq!(solution, 21);
        

    }

    #[test]
    fn test_valve_graph_gets_previous() {
        
        let valves = Valve::from_strings(get_example_input());

        let mut graph = ValveGraph::new(valves.clone());

        let previous_aa_closed = graph.get_previous_graphs("AA");
        assert_eq!(previous_aa_closed.len(), 7); // Open it or come from each adjacent (3) as open or closed.

        graph.open("BB");

        let previous_aa_closed_with_bb_open = graph.get_previous_graphs("AA");
        assert_eq!(previous_aa_closed_with_bb_open.len(), 6); // Open it or come from each adjacent (3) as open or closed, except BB which was open

        graph.open("AA");

        let previous_aa_open_with_bb_open = graph.get_previous_graphs("AA");
        assert_eq!(previous_aa_open_with_bb_open.len(), 5); // Come from each adjacent (3) as open or closed, except BB which was open

    }

    #[test]
    fn test_valve_graph_makes_move() {
        
        let valves = Valve::from_strings(get_example_input());

        let mut graph = ValveGraph::new(valves.clone());

        let walked = graph.make_move("AA", Move::Walk("BB".to_string()));
        assert_eq!(graph, walked.0);
        assert_eq!(walked.1, "BB".to_string());

        let opened = walked.0.make_move("BB", Move::Open);
        assert_ne!(graph, opened.0);
        graph.open("BB");
        assert_eq!(graph, opened.0);
        assert_eq!(opened.1, "BB".to_string());

    }

    #[test]
    fn test_valve_graph_initializes() {
        
        let valves = Valve::from_strings(get_example_input());

        let mut graph = ValveGraph::new(valves.clone());
        assert_eq!(graph.len(), 10);

        assert_eq!(graph.get_neighbors("AA"), Some(vec!["DD".to_string(), "II".to_string(), "BB".to_string()]));
        assert_eq!(graph.get_neighbors("JJ"), Some(vec!["II".to_string()]));

        assert!(graph.is_closed("AA").unwrap());
        assert!(graph.is_closed("BB").unwrap());
        assert!(graph.is_closed("CC").unwrap());
        assert!(graph.is_closed("DD").unwrap());
        
        graph.open("AA");
        assert!(graph.is_open("AA").unwrap());
        
        graph.open_valves(vec!["BB", "CC", "DD"]);
        assert!(graph.is_open("BB").unwrap());
        assert!(graph.is_open("CC").unwrap());
        assert!(graph.is_open("DD").unwrap());

        assert_eq!(
            graph.get_moves("AA"),
            vec![
                Move::Walk("DD".to_string()), 
                Move::Walk("II".to_string()), 
                Move::Walk("BB".to_string()), 
            ]
        );

        assert_eq!(
            graph.get_moves("JJ"),
            vec![
                Move::Open,
                Move::Walk("II".to_string()), 
            ]
        );

        assert_eq!(graph.get_cumulative_pressure_from(30), 0);
        assert_eq!(graph.get_cumulative_pressure_from(29), 13 + 2 + 20);
        assert_eq!(graph.get_cumulative_pressure_from(28), (13 + 2 + 20) * 2);
        assert_eq!(graph.get_cumulative_pressure_from(0), (13 + 2 + 20) * 30);
        
    }

    // #[test]
    // fn test_valve_graph_generates_state() {
        
    //     let valves = Valve::from_strings(get_example_input());
    //     let mut graph = ValveGraph::new(valves.clone());

    //     let initial_graph_state = graph.get_state(0, "AA");
    //     graph.open("AA");

    //     assert_ne!(initial_graph_state, graph.get_state(0, "AA"));

    //     let t_29_state = graph.get_state(29, "AA");
    //     graph.open("JJ");

    //     assert_eq!(t_29_state, graph.get_state(29, "AA"));

    //     let t_28_state = graph.get_state(28, "AA");
    //     graph.open("BB");

    //     assert_eq!(t_28_state, graph.get_state(28, "AA"));
        
    //     let t_27_state = graph.get_state(27, "AA");

    //     graph.open("II");

    //     assert_ne!(t_27_state, graph.get_state(28, "AA"));

    // }

    #[test]
    fn test_valve_graph_gets_open_indices_within() {
        
        let valves = Valve::from_strings(get_example_input());
        let mut graph = ValveGraph::new(valves.clone());

        assert_eq!(graph.get_open_indices_within(0, "AA"), vec![].into_iter().collect());
        graph.open("AA");

        assert_eq!(graph.get_open_indices_within(0, "AA"), vec![0].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(1, "AA"), vec![0].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(5, "AA"), vec![0].into_iter().collect());
        
        graph.open("BB");
        assert_eq!(graph.get_open_indices_within(0, "AA"), vec![0].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(1, "AA"), vec![0, 1].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(5, "AA"), vec![0, 1].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(0, "BB"), vec![1].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(1, "BB"), vec![1, 0].into_iter().collect());

        graph.open("DD");
        assert_eq!(graph.get_open_indices_within(0, "AA"), vec![0].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(1, "AA"), vec![0, 1, 3].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(5, "AA"), vec![0, 1, 3].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(0, "BB"), vec![1].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(1, "BB"), vec![1, 0].into_iter().collect());
        assert_eq!(graph.get_open_indices_within(2, "BB"), vec![1, 0, 3].into_iter().collect());
        
    }

    #[test]
    fn test_parsing() {

        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        // Valve BB has flow rate=13; tunnels lead to valves CC, AA
        // Valve CC has flow rate=2; tunnels lead to valves DD, BB
        // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        // Valve EE has flow rate=3; tunnels lead to valves FF, DD
        // Valve FF has flow rate=0; tunnels lead to valves EE, GG
        // Valve GG has flow rate=0; tunnels lead to valves FF, HH
        // Valve HH has flow rate=22; tunnel leads to valve GG
        // Valve II has flow rate=0; tunnels lead to valves AA, JJ
        // Valve JJ has flow rate=21; tunnel leads to valve II

        let valves = Valve::from_strings(get_example_input());
        assert_eq!(valves.len(), 10);

        assert_eq!(valves[0].get_label(), "AA".to_string());
        assert_eq!(valves[0].get_flow_rate(), 0);
        assert_eq!(valves[0].get_neighbors(), &vec!["DD".to_string(), "II".to_string(), "BB".to_string()]);

        assert_eq!(valves[9].get_label(), "JJ".to_string());
        assert_eq!(valves[9].get_flow_rate(), 21);
        assert_eq!(valves[9].get_neighbors(), &vec!["II".to_string()]);

        assert!(valves.iter().all(|v| v.is_closed()));

        let mut valve = valves[0].clone();
        valve.open();

        assert!(!valve.is_closed());
        assert!(valve.is_open());

        assert_eq!(valve.get_cumulative_pressure_from(30), 0);
        
        let mut valve = valves[9].clone();
        valve.open();

        assert_eq!(valve.get_cumulative_pressure_from(30), 0);
        assert_eq!(valve.get_cumulative_pressure_from(29), 21);
        assert_eq!(valve.get_cumulative_pressure_from(28), 42);
        assert_eq!(valve.get_cumulative_pressure_from(1), 29 * 21);

        assert_eq!(valves[2].get_cumulative_pressure_from(0), 0);
        
    }

    #[test]
    fn test_valve_graph_updates_from_state() {
        let valves = Valve::from_strings(get_example_input());
        let graph = ValveGraph::new(valves.clone());

        let mut cloned = graph.clone();
        cloned.open("BB");
        cloned.open("JJ");
        assert_eq!(cloned.get_open_indices(), vec![1, 9].into_iter().collect());

        let state = cloned.get_state(13, "CC");
        let from_state = graph.update_from_state(state);

        assert_eq!(from_state, cloned);
        
    }

    #[test]
    fn test_valve_graph_state_gets_permutations() {
        
        let valves = Valve::from_strings(get_example_input());
        let graph = ValveGraph::new(valves.clone());

        let state = graph.get_state(29, "AA");
        assert_eq!(state.get_all_permutations(vec![]).len(), 1);
        assert_eq!(state.get_all_permutations(vec![0]).len(), 2);
        assert_eq!(state.get_all_permutations(vec![0, 1]).len(), 4);
        assert_eq!(state.get_all_permutations(vec![5, 6]).len(), 4);

    }

    #[test]
    fn test_split_splits_on_multiple_characters() {
        let neighbors_string: &str = "DD, II, BB";
        let neighbors = neighbors_string.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

        assert_eq!(neighbors, vec!["DD".to_string(), "II".to_string(), "BB".to_string()]);

    }
}