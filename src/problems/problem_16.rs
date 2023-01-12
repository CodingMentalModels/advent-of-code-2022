use std::{collections::{HashMap, HashSet}, thread::current};
use itertools::Itertools;

use crate::input::input::InputParser;

type Time = usize;

const INITIAL_MINUTES_UNTIL_ERUPTION: Time = 30;
const MINUTES_SPENT_TEACHING_ELEPHANT: Time = 4;
const DISTANCE_BIGGER_THAN_MAX: usize = 1000;

pub fn solve_problem_16a(input: Vec<String>) -> u32 {

        let valves = Valve::from_strings(input);
        let graph = ValveGraph::new(valves.clone());

        let nodes_with_flow = (0..valves.len()).into_iter().filter(|i| valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();
        graph.get_maximum_flow(&nodes_with_flow, INITIAL_MINUTES_UNTIL_ERUPTION, graph.get_index("AA").unwrap(), 0)
}

fn solve_problem_16b(input: Vec<String>) -> u32 {
        let valves = Valve::from_strings(input);
        let graph = ValveGraph::new(valves.clone());

        let nodes_with_flow = (0..valves.len()).into_iter().filter(|i| valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let time_remaining = INITIAL_MINUTES_UNTIL_ERUPTION - MINUTES_SPENT_TEACHING_ELEPHANT;

        graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            time_remaining,
            Plan::new(graph.get_index("AA").unwrap(), 0),
            Plan::new(graph.get_index("AA").unwrap(), 0),
            0
        )
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct ValveGraph {
    ordering: HashMap<String, usize>,
    valves: Vec<Valve>,
    distances: HashMap<(usize, usize), usize>,
}

impl ValveGraph {

    pub fn new(valves: Vec<Valve>) -> Self {
        let labels = valves.iter().enumerate().map(|(i, v)| (v.get_label(), i)).collect();
        let distances = Self::compute_distances(valves.clone());
        ValveGraph { ordering: labels, valves: valves, distances}
    }

    pub fn get_valve(&self, label: &str) -> Option<Valve> {
        self.ordering.get(label).map(|i| self.valves[*i].clone())
    }

    pub fn get_index(&self, label: &str) -> Option<usize> {
        self.ordering.get(label).copied()
    }

    pub fn index_to_label(&self, i: usize) -> String {
        self.valves[i].get_label()
    }

    pub fn get_labels(&self) -> Vec<String> {
        self.valves.iter().map(|v| v.get_label()).collect()
    }

    pub fn get_neighbors(&self, label: &str) -> Option<Vec<String>> {
        self.get_valve(label)
            .map(
                |v| v.get_neighbors().clone()
            )
    }

    pub fn get_distance(&self, i_label: &str, j_label: &str) -> Option<usize> {
        let i = self.ordering.get(i_label);
        let j = self.ordering.get(j_label);

        if i.is_none() || j.is_none() {
            return None;
        } else if i == j {
            return Some(0);
        }
        Some(*self.distances.get(&(*i.unwrap(), *j.unwrap())).expect("If the labels exist, they should already have distances computed."))
    }

    pub fn len(&self) -> usize {
        self.valves.len()
    }

    pub fn compute_distances(valves: Vec<Valve>) -> HashMap<(usize, usize), usize> {
        let n_nodes = valves.len();
        let edges: Vec<(usize, usize)> = (0..n_nodes).into_iter().permutations(2).map(|v| (v[0], v[1])).collect();
        let mut distances: HashMap<(usize, usize), usize> = edges.into_iter().map(|(i, j)| {
                if valves[i].get_neighbors().contains(&valves[j].get_label()) {
                    ((i, j), 1)
                } else {
                    ((i, j), DISTANCE_BIGGER_THAN_MAX)
                }
        }).collect();
        let permutations: Vec<(usize, usize, usize)> = (0..n_nodes).into_iter().permutations(3).map(|v| (v[0], v[1], v[2])).collect();
        for (k, i, j ) in permutations {
            // We go through the k's first so that we solve all k = 0, then k = 1, etc. recursively.
            let old_distance = *distances.get(&(i, j)).unwrap();
            let candidate = distances.get(&(i, k)).unwrap() + distances.get(&(k, j)).unwrap();
            let new_distance = if candidate < old_distance {
                candidate
            } else {
                old_distance
            };
            distances.insert((i, j), new_distance);
        }
        
        return distances;
    }

    pub fn get_maximum_flow(&self, unvisited: &HashSet<usize>, time_remaining: Time, current_position: usize, max_so_far: u32) -> u32 {
        let new_max_so_far = max_so_far + self.valves[current_position].get_cumulative_pressure_if_opened_over(time_remaining);
        if time_remaining == 0 || unvisited.len() == 0 {
            return new_max_so_far;
        }

        unvisited.iter()
            .filter(|n| *self.distances.get(&(current_position, **n)).unwrap() + 1 <= time_remaining)
            .map(|n| {
                let new_unvisited = unvisited.iter().cloned().filter(|i| i != n).collect::<HashSet<_>>();
                let new_time_remaining = time_remaining - self.distances.get(&(current_position, *n)).unwrap() - 1;
                self.get_maximum_flow(
                    &new_unvisited,
                    new_time_remaining,
                    *n,
                    new_max_so_far,
                )
            }
        ).max().unwrap_or(new_max_so_far)

    }

    pub fn get_maximum_flow_with_elephant(&self, unvisited: &HashSet<usize>, time_remaining: Time, player_plan: Plan, elephant_plan: Plan, max_so_far: u32) -> u32 {
        let mut new_max_so_far = max_so_far;
        if player_plan.arrived() {
            new_max_so_far += self.valves[player_plan.target].get_cumulative_pressure_if_opened_over(time_remaining);
        }
        if elephant_plan.arrived() {
            new_max_so_far += self.valves[elephant_plan.target].get_cumulative_pressure_if_opened_over(time_remaining);
        }

        if time_remaining == 0 {
            return new_max_so_far;
        }

        if player_plan.arrived() && elephant_plan.arrived() {
            let to_visit = unvisited.iter().permutations(2).map(|v| (*v[0], *v[1])).collect();
            self.visit_with_elephant(unvisited, time_remaining, player_plan, elephant_plan, new_max_so_far, to_visit)
        } else if player_plan.arrived() {
            let to_visit = unvisited.iter().map(|i| (*i, elephant_plan.target)).collect();
            self.visit_with_elephant(unvisited, time_remaining, player_plan, elephant_plan, new_max_so_far, to_visit)
        } else if elephant_plan.arrived() {
            let to_visit = unvisited.iter().map(|i| (player_plan.target, *i)).collect();
            self.visit_with_elephant(unvisited, time_remaining, player_plan, elephant_plan, new_max_so_far, to_visit)
        } else {
            let to_visit = vec![(player_plan.target, elephant_plan.target)].into_iter().collect();
            self.visit_with_elephant(unvisited, time_remaining, player_plan, elephant_plan, new_max_so_far, to_visit)
        }

    }

    fn visit_with_elephant(
        &self,
        unvisited: &HashSet<usize>,
        time_remaining: Time,
        player_plan: Plan,
        elephant_plan: Plan,
        new_max_so_far: u32,
        to_visit: HashSet<(usize, usize)>
    ) -> u32 {
        let new_time_remaining = time_remaining - 1;
        return to_visit.into_iter()
            .map(|(player_target, elephant_target)| {
                assert_ne!(player_target, elephant_target);
                let new_unvisited = unvisited.iter().cloned()
                    .filter(|i| *i != player_target && *i != elephant_target).collect::<HashSet<_>>();
                let new_player_plan = if player_plan.arrived() {
                    Plan::new(player_target, *self.distances.get(&(player_plan.target, player_target)).unwrap())
                } else {
                    player_plan.get_ticked()
                };
                let new_elephant_plan = if elephant_plan.arrived() {
                    Plan::new(elephant_target, *self.distances.get(&(elephant_plan.target, elephant_target)).unwrap())
                } else {
                    elephant_plan.get_ticked()
                };

                self.get_maximum_flow_with_elephant(
                    &new_unvisited,
                    new_time_remaining,
                    new_player_plan,
                    new_elephant_plan,
                    new_max_so_far,
                )
            }
        ).max().unwrap_or(new_max_so_far);
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
struct Plan {
    target: usize,
    time_remaining: Time,
}

impl Plan {

    pub fn new(target: usize, time_remaining: Time) -> Self {
        Self {target, time_remaining}
    }

    pub fn get_ticked(&self) -> Self {
        let time_remaining = if self.time_remaining == 0 { 0 } else { self.time_remaining - 1 };
        Self::new(self.target, time_remaining)
    }

    pub fn arrived(&self) -> bool {
        self.time_remaining == 0
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

    pub fn linear(flows: Vec<u32>) -> Vec<Self> {
        assert!(flows.len() >= 2);
        (0..flows.len()).map(
            |i| Valve::new(
                flows[i].to_string(),
                flows[i],
                if i == 0 {
                    vec![flows[1].to_string()]
                } else if i == flows.len() - 1 {
                    vec![flows[i - 1].to_string()]
                } else {
                    vec![flows[i - 1].to_string(), flows[i + 1].to_string()]
                }
            )
        ).collect()
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

    pub fn get_cumulative_pressure_from(&self, t: Time) -> u32 {
        if self.is_open {
            (INITIAL_MINUTES_UNTIL_ERUPTION - t) as u32 * self.flow_rate
        } else { 0 }
    }

    pub fn get_cumulative_pressure_if_opened_over(&self, time_remaining: Time) -> u32 {
        time_remaining as u32 * self.flow_rate
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
        
        let input = InputParser::new().parse_as_string("input_16.txt").unwrap();

        let answer = solve_problem_16a(input);
        assert_eq!(answer, 2183);
    }
    
    #[test]
    fn test_problem_16b_passes() {
        assert_eq!(solve_problem_16b(get_example_input()), 1707);

        let input = InputParser::new().parse_as_string("input_16.txt").unwrap();

        let answer = solve_problem_16b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_computes_path_with_elephant() {

        let valves = Valve::from_strings(get_example_input());

        let graph = ValveGraph::new(valves.clone());

        let nodes_with_flow = (0..valves.len()).into_iter().filter(|i| valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            2,
            Plan::new(graph.get_index("AA").unwrap(), 0),
            Plan::new(graph.get_index("AA").unwrap(), 0),
            0
        );

        assert_eq!(solution, 0);

        let graph = ValveGraph::new(valves.clone());

        let nodes_with_flow = (0..valves.len()).into_iter().filter(|i| valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            3,
            Plan::new(graph.get_index("AA").unwrap(), 0),
            Plan::new(graph.get_index("AA").unwrap(), 0),
            0
        );

        assert_eq!(solution, 13 + 20);

        let graph = ValveGraph::new(valves.clone());

        let nodes_with_flow = (0..valves.len()).into_iter().filter(|i| valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            5,
            Plan::new(graph.get_index("AA").unwrap(), 0),
            Plan::new(graph.get_index("AA").unwrap(), 0),
            0
        );

        assert_eq!(solution, 20*3 + 21*2); // NB: Greater than 20*3 + 13*3
        
    }

    #[test]
    fn test_simple_valve_graph_finds_maximum_flow_with_elephant() {
        
        let simple_valves = Valve::linear(vec![0, 1, 10, 100, 1000]);
        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            2,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 0);

        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            3,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 1);

        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            4,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 2*1 + 10);

        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            5,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 2*10 + 100);

        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            6,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 2*100 + 1000);

        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            7,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 3*100 + 2*1000);

        let simple_graph = ValveGraph::new(simple_valves.clone());

        let nodes_with_flow = (0..simple_valves.len()).into_iter().filter(|i| simple_valves[*i].get_flow_rate() > 0).collect::<HashSet<_>>();

        let solution = simple_graph.get_maximum_flow_with_elephant(
            &nodes_with_flow,
            100,
            Plan::new(0, 0),
            Plan::new(0, 0),
            0
        );

        assert_eq!(solution, 96*100 + 95*1000);
    }

    #[test]
    fn test_valve_graph_initializes() {
        
        let valves = Valve::from_strings(get_example_input());

        let mut graph = ValveGraph::new(valves.clone());
        assert_eq!(graph.len(), 10);

        assert_eq!(graph.get_neighbors("AA"), Some(vec!["DD".to_string(), "II".to_string(), "BB".to_string()]));
        assert_eq!(graph.get_neighbors("JJ"), Some(vec!["II".to_string()]));

        assert_eq!(graph.get_distance("AA", "AA").unwrap(), 0);
        assert_eq!(graph.get_distance("AA", "DD").unwrap(), 1);
        assert_eq!(graph.get_distance("AA", "FF").unwrap(), 3);
        assert_eq!(graph.get_distance("JJ", "FF").unwrap(), 5);
        assert_eq!(graph.get_distance("FF", "JJ").unwrap(), 5);

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
    }

    #[test]
    fn test_valves_insantiate_linearly() {
       let valves = Valve::linear(vec![0, 1, 10, 100, 1000]);
       assert_eq!(valves.len(), 5);

       assert_eq!(valves.iter().map(|v| v.get_label()).collect::<Vec<_>>(),
            vec![
                "0".to_string(),
                "1".to_string(),
                "10".to_string(),
                "100".to_string(),
                "1000".to_string()
            ]
        );
       assert_eq!(valves.iter().map(|v| v.get_flow_rate()).collect::<Vec<_>>(), vec![0, 1, 10, 100, 1000]);
       assert_eq!(valves[0].get_neighbors(), &vec!["1".to_string()]);
       assert_eq!(valves[2].get_neighbors(), &vec!["1".to_string(), "100".to_string()]);
       assert_eq!(valves[4].get_neighbors(), &vec!["100".to_string()]);
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
    fn test_split_splits_on_multiple_characters() {
        let neighbors_string: &str = "DD, II, BB";
        let neighbors = neighbors_string.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

        assert_eq!(neighbors, vec!["DD".to_string(), "II".to_string(), "BB".to_string()]);

    }
}