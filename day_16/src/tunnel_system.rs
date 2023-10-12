use std::{collections::{HashMap, VecDeque, HashSet, BinaryHeap, BTreeSet, BTreeMap}};
use itertools::Itertools;

pub struct Node<'a> {
    label: &'a str,
    flow_rate: i32
}

//Essentially an undirected graph.
pub struct TunnelSystem<'a> {
    nodes: Vec<Node<'a>>,
    edges: Vec<Vec<usize>>
}

struct AnalysisState {
    current_node: usize,
    opened_valves: BTreeSet<usize>,
    pressure_released: i32,
    current_time_step: i32
}

impl<'a> TunnelSystem<'a> {
    pub fn find_node_by_label(&self, label: &str) -> Option<(usize, &Node)> {
        self.nodes.iter().enumerate().find(|n| n.1.label == label)
    }

    fn add_edge(&mut self, first_label: &str, second_label: &str) {
        //Adds an edge between nodes n and m.
        let first_node_with_index = self.find_node_by_label(first_label);
        let second_node_with_index =self.find_node_by_label(second_label);

        match (first_node_with_index, second_node_with_index) {
            (Some(first), Some(second)) => {
                let (first_index, _) = first;
                let (second_index, _) = second;

                if !self.edges[first_index].contains(&second_index) {
                    self.edges[first_index].push(second_index);
                    self.edges[second_index].push(first_index);
                }
            },
            _ => {}
        }
    }

    pub fn find_maximum_release(&self, start_index: usize, max_time_steps: i32) -> HashMap<BTreeSet<usize>, i32> {
        let distance_matrix = self.build_distance_matrix(start_index);
        let flowing_valves: Vec<usize> = self.nodes.iter().enumerate().filter(|(_, valve)| valve.flow_rate > 0).map(|(index, _)| index).collect();

        let mut max_released = 0;
        let mut valves_opened: BTreeSet<usize> = BTreeSet::new();

        let mut states = VecDeque::new();
        let mut seen: HashSet<(BTreeSet<usize>, i32, i32)> = HashSet::new();
        states.push_back(AnalysisState{
            opened_valves: BTreeSet::new(),
            current_node: start_index,
            pressure_released: 0,
            current_time_step: 0
        }); 

        seen.insert((BTreeSet::new(), 0, 0));

        let mut best_states = HashMap::new();

        while let Some(AnalysisState { 
            current_node, 
            opened_valves, 
            pressure_released, 
            current_time_step 
        }) = states.pop_front() {
            let pressure_released_this_turn = opened_valves.iter().map(|i| self.nodes[*i].flow_rate).sum::<i32>();
            let time_remaining = max_time_steps - current_time_step;
            let new_pressure = pressure_released + (pressure_released_this_turn * time_remaining);

            best_states.entry(opened_valves.clone())
                .and_modify(|val| *val = new_pressure.max(*val))
                .or_insert(new_pressure);

            if flowing_valves.len() == opened_valves.len() {
                continue;
            }
            
            let unopened_valves = flowing_valves.iter().filter(|index| !opened_valves.contains(index));
            for unopened_valve_index in unopened_valves {
                let distance = distance_matrix.get(&(current_node, *unopened_valve_index)).unwrap();
                let cost = *distance + 1;
                
                let new_time_step = current_time_step + cost;   
                if new_time_step >= max_time_steps {
                    continue;
                }
                
                let new_pressure_released = pressure_released + (pressure_released_this_turn * cost);
                let mut new_opened_valves = opened_valves.clone();
                new_opened_valves.insert(*unopened_valve_index);

                if seen.insert((new_opened_valves.clone(), new_time_step, new_pressure_released)) {
                    states.push_back(AnalysisState {
                        current_node: *unopened_valve_index,
                        opened_valves: new_opened_valves,
                        pressure_released: new_pressure_released,
                        current_time_step: new_time_step
                    })
                }
            }
        }
        
        best_states
    }

    pub fn new(input: Vec<((&'a str, i32), Vec<String>)>) -> Self {
        let mut result = TunnelSystem { 
            nodes: vec![],
            edges: vec![]
        }; 

        //May not be the most efficient approach, but we construct the tunnel system by first adding all of the nodes themselves,
        //and then we construct the edges in a separate pass.
        for (node, _) in input.iter() {
            result.nodes.push(Node { label: node.0, flow_rate: node.1 });
            result.edges.push(vec![])
        }

        for (node, adjacency_list) in input.iter() {
            for second in adjacency_list {
                result.add_edge(node.0, second)
            }
        }

        result
    }

    fn build_distance_matrix(&self, start_idx: usize) -> HashMap<(usize, usize), i32> {
        let flowing_nodes = self.nodes
            .iter()
            .enumerate()
            .filter(|(index, x) | x.flow_rate > 0 || *index == start_idx);

        let distances = flowing_nodes.tuple_combinations().map(|(first, second)| {
            let (first_idx, _) = first;
            let (second_idx, _) = second;

            let distance = self.min_distance(first_idx, second_idx);
            [
                ((first_idx, second_idx), distance), 
                ((second_idx, first_idx), distance)
            ]
        });

        distances.flatten().collect::<HashMap<_, _>>()
    }

    pub fn min_distance(&self, start: usize, end: usize) -> i32 {
        //We exploit lexicographical sorting, so cost goes first. BinaryHeaps are always sorted least to greatest, so this allows us to return as soon as we see our value.
        //We also have to keep in mind that BinaryHeap sorts greatest -> lowest, so we do a negative cost to get a min heap, and then revert
        let mut queue: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        queue.push((0, start));

        let mut seen: HashSet<usize> = HashSet::new();

        while let Some((cost, node)) = queue.pop() {
            if node == end {
                return -cost
            }

            for neighbor in &self.edges[node] {
                if seen.insert(*neighbor) {
                    queue.push((cost - 1, *neighbor))
                }
            }
        }
        0
    }
}