use std::collections::{HashMap, VecDeque};
use std::{fs};
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;
use std::cmp::max;
use std::time::SystemTime;

type NodeIndex = usize;
type EdgeIndex = usize;

struct NodeData {
    flow_rate: u32,
    first_outgoing_edge: Option<EdgeIndex>
}

struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    fn add_node(&mut self, flow_rate: u32) -> NodeIndex {
        // to do: duplicate check
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            flow_rate,
            first_outgoing_edge: None
        });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        // to do: duplicate check
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors {
            graph: self,
            current_edge_index: first_outgoing_edge
        }
    }
}

pub struct Volcano {
    graph: Graph,
    start_node: NodeIndex,
    path_map: HashMap<(NodeIndex, NodeIndex), u32>,
    minutes_left: u32
}

impl Volcano {
    pub fn new(volcano_map: HashMap<String, (u32, Vec<String>)>, minutes_left: u32) -> Result<Self, &'static str> {
        let mut graph = Graph::new();
        let mut node_index_mapping = HashMap::new();
        // add all nodes
        for (name, (flow_rate, _)) in &volcano_map {
            let index = graph.add_node(*flow_rate);
            node_index_mapping.insert(name.clone(), index);
        }
        // set start node (Err if not exists)
        if let Some(node) = node_index_mapping.get("AA") {
            // add all edges (err if a node does not exist)
            let start_node = *node;
            for (name, (_, neighbour_vec)) in volcano_map {
                for neighbour_name in neighbour_vec {
                    if node_index_mapping.contains_key(&name) && node_index_mapping.contains_key(&neighbour_name) {
                        graph.add_edge(*node_index_mapping.get(&name).unwrap(), *node_index_mapping.get(&neighbour_name).unwrap());
                    } else {
                        return Err("Invalid Edge - At least on node does not exist")
                    }
                }
            }
            Ok(
                Volcano {
                    graph,
                    start_node,
                    path_map: HashMap::new(),
                    minutes_left
                }
            )
        } else {
            Err("volcano_map has no start valve (AA)")
        }
    }

    pub fn brute_force_path_search(&mut self) -> u32{
        println!("Solving Brute Force");

        // Add all relevant nodes
        let mut relevant_nodes = Vec::new();
        for i in 0..self.graph.nodes.len() {
            if 0 < self.graph.nodes[i].flow_rate {
                relevant_nodes.push(i);
            }
        }

        // calc all paths
        println!("Starting Calc of all paths.");
        let start = SystemTime::now();
        self.path_map = self.calc_shortest_paths(&relevant_nodes, 8);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Time went backwards.");
        println!("Duration: {:?}", duration);

        // calc all permutations
        println!("Generating all relevant permutations.");
        let start = SystemTime::now();
        let permutations = self.generate_all_permutations(&relevant_nodes, 8);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Time went backwards.");
        println!("Duration: {:?}", duration);

        // walk all paths
        println!("Walking all paths.");
        let start = SystemTime::now();
        let best = self.walk_all_potential_paths(permutations, 8);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Time went backwards.");
        println!("Duration: {:?}", duration);

        best
    }

    fn calc_shortest_paths(&self, relevant_nodes: &Vec<usize>, number_of_workers: u32) -> HashMap<(NodeIndex, NodeIndex), u32> {
        let mut shortest_paths = HashMap::new();
        let (snd1, rcv1) = crossbeam_channel::bounded(1);
        let (snd2, rcv2) = crossbeam_channel::bounded(1);

        crossbeam::scope(|scope| {
            // Producer thread
            scope.spawn(|_| {
                for node in relevant_nodes {
                    snd1.send((self.start_node, *node)).unwrap();
                    for n in relevant_nodes {
                        snd1.send((*node, *n)).unwrap();
                    }
                }
                // Close the channel - this is necessary to exit
                // the for-loop in the worker
                drop(snd1);
            });

            // Parallel processing
            for _ in 0..number_of_workers {
                // send to sink, receive from source
                let (sendr, recvr) = (snd2.clone(), rcv1.clone());
                // spawn worker in separate threads
                scope.spawn(move |_| {
                    for message in recvr.iter() {
                        let val = self.shortest_path(message.0, message.1).unwrap();
                        sendr.send((message, val)).unwrap();
                    }
                });
            }

            // Close the channel, otherwise sink will never
            // exit the for-loop
            drop(snd2);

            // Sink
            for msg in rcv2.iter() {
                shortest_paths.insert(msg.0, msg.1);
            }
        }).unwrap();

        shortest_paths
    }

    fn shortest_path(&self, current_node: NodeIndex, target_node: NodeIndex) -> Option<u32> {
        let mut visited_nodes = HashMap::new();
        let mut queue= VecDeque::new();
        visited_nodes.insert(current_node, true);
        queue.push_front((current_node, 0));
        while !queue.is_empty() {
            let (n_index, minutes) = queue.pop_back().unwrap();
            if n_index == target_node {
                return Some(minutes)
            } else {
                for neighbour in self.graph.successors(n_index) {
                    if !visited_nodes.contains_key(&neighbour) {
                        visited_nodes.insert(neighbour, true);
                        queue.push_front((neighbour, minutes + 1));
                    }
                }
            }
        }
        None
    }

    fn generate_all_permutations(&self, relevant_nodes: &Vec<NodeIndex>, number_of_workers: u32) -> Vec<Vec<NodeIndex>> {
        let mut permutations_vec = Vec::new();
        let (snd1, rcv1) = crossbeam_channel::bounded(1);
        let (snd2, rcv2) = crossbeam_channel::bounded(1);

        crossbeam::scope(|scope| {
            // Producer thread
            scope.spawn(|_| {
                for node_index in relevant_nodes {
                    let permutation_vec = vec![self.start_node, *node_index];
                    let minutes_left = self.minutes_left - self.path_map.get(&(self.start_node, *node_index)).unwrap() - 1;
                    snd1.send((permutation_vec, minutes_left)).unwrap();
                }
                // Close the channel - this is necessary to exit
                // the for-loop in the worker
                drop(snd1);
            });

            // Parallel processing
            for _ in 0..number_of_workers {
                // send to sink, receive from source
                let (sendr, recvr) = (snd2.clone(), rcv1.clone());
                // spawn worker in separate threads
                scope.spawn(move |_| {
                    for message in recvr.iter() {
                        let permutations_vec = self.bfs_all_permutations(message.0, relevant_nodes, message.1);
                        sendr.send(permutations_vec).unwrap();
                    }
                });
            }

            // Close the channel, otherwise sink will never
            // exit the for-loop
            drop(snd2);

            // Sink
            for mut msg in rcv2.iter() {
                permutations_vec.append(&mut msg);
            }
        }).unwrap();

        permutations_vec
    }

    fn bfs_all_permutations(&self, permutation_vec: Vec<NodeIndex>, relevant_nodes: &Vec<NodeIndex>, minutes_left: u32) -> Vec<Vec<NodeIndex>> {
        let mut queue = VecDeque::new();
        queue.push_front((permutation_vec, minutes_left));
        let mut permutations_vec = Vec::new();
        while !queue.is_empty() {
            let (permutation_vec, minutes_left) = queue.pop_back().unwrap();
            let mut nothing_got_added = true;
            for node in relevant_nodes {
                if !permutation_vec.contains(node) {
                    let cost = self.path_map.get(&(*permutation_vec.last().unwrap(), *node)).unwrap() + 1;
                    if cost < minutes_left {
                        let mut new_permutation_vec = permutation_vec.clone();
                        new_permutation_vec.push(*node);
                        queue.push_front((new_permutation_vec, minutes_left - cost));
                        nothing_got_added = false;
                    }
                }
            }
            if nothing_got_added {
                permutations_vec.push(permutation_vec)
            }
        }
        permutations_vec
    }

    fn walk_all_potential_paths(&self, permutations: Vec<Vec<NodeIndex>>, number_of_workers: u32) -> u32 {
        println!("Number of possibilities: {}", permutations.len());
        let mut best = 0;
        let (snd1, rcv1) = crossbeam_channel::bounded(1);
        let (snd2, rcv2) = crossbeam_channel::bounded(1);

        crossbeam::scope(|scope| {
            // Producer thread
            scope.spawn(|_| {
                for path in &permutations {
                    snd1.send(path).unwrap();
                }
                // Close the channel - this is necessary to exit
                // the for-loop in the worker
                drop(snd1);
            });

            // Parallel processing
            for _ in 0..number_of_workers {
                // send to sink, receive from source
                let (sendr, recvr) = (snd2.clone(), rcv1.clone());
                // spawn worker in separate threads
                scope.spawn(move |_| {
                    for message in recvr.iter() {
                        let val = self.walk_routine(message);
                        sendr.send(val).unwrap();
                    }
                });
            }

            // Close the channel, otherwise sink will never
            // exit the for-loop
            drop(snd2);

            // Sink
            for msg in rcv2.iter() {
                best = max(msg, best);
            }
        }).unwrap();

        best
    }

    fn walk_routine(&self, path: &Vec<NodeIndex>) -> u32 {
        let mut pressure_released = 0;
        let mut minutes: u32 = self.minutes_left;
        for i in 0..path.len()-1 {
            minutes -= self.path_map.get(&(path[i], path[i+1])).unwrap() + 1;
            pressure_released += self.graph.nodes[path[i+1]].flow_rate * minutes;
        }
        pressure_released
    }


    pub fn brute_force_path_search_with_elephant(&mut self) -> u32{
        println!("Solving Brute Force");

        // Add all relevant nodes
        let mut relevant_nodes = Vec::new();
        for i in 0..self.graph.nodes.len() {
            if 0 < self.graph.nodes[i].flow_rate {
                relevant_nodes.push(i);
            }
        }

        // calc all paths
        println!("Starting Calc of all paths.");
        let start = SystemTime::now();
        self.path_map = self.calc_shortest_paths(&relevant_nodes, 8);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Time went backwards.");
        println!("Duration: {:?}", duration);

        // calc all permutations
        println!("Generating all relevant permutations.");
        let start = SystemTime::now();
        let permutations = self.generate_all_permutations(&relevant_nodes, 8);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Time went backwards.");
        println!("Duration: {:?}", duration);

        // walk all paths
        println!("Walking all paths.");
        let start = SystemTime::now();
        let best = self.walk_all_potential_paths_with_elephant(permutations, 8);
        let end = SystemTime::now();
        let duration = end.duration_since(start).expect("Time went backwards.");
        println!("Duration: {:?}", duration);

        best
    }
/*
    fn generate_all_permutations_with_elephant(&self, relevant_nodes: &Vec<NodeIndex>, number_of_workers: u32) -> Vec<(Vec<NodeIndex>, Vec<NodeIndex>)> {
        let mut permutations_vec = Vec::new();
        let (snd1, rcv1) = crossbeam_channel::bounded(1);
        let (snd2, rcv2) = crossbeam_channel::bounded(1);

        crossbeam::scope(|scope| {
            // Producer thread
            scope.spawn(|_| {
                for node_index in relevant_nodes {
                    let permutation_vec = vec![self.start_node, *node_index];
                    let minutes_left = self.minutes_left - self.path_map.get(&(self.start_node, *node_index)).unwrap() - 1;
                    snd1.send((permutation_vec, minutes_left)).unwrap();
                }
                // Close the channel - this is necessary to exit
                // the for-loop in the worker
                drop(snd1);
            });

            // Parallel processing
            for _ in 0..number_of_workers {
                // send to sink, receive from source
                let (sendr, recvr) = (snd2.clone(), rcv1.clone());
                // spawn worker in separate threads
                scope.spawn(move |_| {
                    for message in recvr.iter() {
                        let permutations_vec_tuple = self.bfs_all_permutations_with_elephant(message.0, relevant_nodes, message.1);
                        sendr.send(permutations_vec_tuple).unwrap();
                    }
                });
            }

            // Close the channel, otherwise sink will never
            // exit the for-loop
            drop(snd2);

            // Sink
            for mut msg in rcv2.iter() {
                permutations_vec.append(&mut msg);
            }
        }).unwrap();

        permutations_vec
    }

    fn bfs_all_permutations_with_elephant(&self, permutation_vec: Vec<NodeIndex>, relevant_nodes: &Vec<NodeIndex>, minutes_left: u32) -> Vec<(Vec<NodeIndex>, Vec<NodeIndex>)> {
        let mut queue = VecDeque::new();
        queue.push_front(((permutation_vec.clone(), minutes_left), (vec![self.start_node], self.minutes_left)));
        let mut permutations_vec = Vec::new();
        while !queue.is_empty() {
            let ((permutation_vec_1, minutes_left_1), (permutation_vec_2, minutes_left_2)) = queue.pop_back().unwrap();
            let mut nothing_got_added = true;
            for node in relevant_nodes {
                if !permutation_vec_1.contains(node) && !permutation_vec_2.contains(node) {
                    // one permutation where vec 1 takes the node
                    let cost_1 = self.path_map.get(&(*permutation_vec_1.last().unwrap(), *node)).unwrap() + 1;
                    if cost_1 < minutes_left_1 {
                        let mut new_permutation_vec_1 = permutation_vec_1.clone();
                        new_permutation_vec_1.push(*node);
                        queue.push_front(((new_permutation_vec_1, minutes_left_1 - cost_1),(permutation_vec_2.clone(), minutes_left_2)));
                        nothing_got_added = false;
                    }
                    // one permutation where vec 2 takes the node
                    let cost_2 = self.path_map.get(&(*permutation_vec_2.last().unwrap(), *node)).unwrap() + 1;
                    if cost_2 < minutes_left_2 {
                        let mut new_permutation_vec_2 = permutation_vec_2.clone();
                        new_permutation_vec_2.push(*node);
                        queue.push_front(((permutation_vec_1.clone(), minutes_left_1),(new_permutation_vec_2, minutes_left_2 - cost_2)));
                        nothing_got_added = false;
                    }
                }
            }
            if nothing_got_added {
                permutations_vec.push((permutation_vec_1, permutation_vec_2))
            }
        }
        permutations_vec
    }
*/
    fn walk_all_potential_paths_with_elephant(&self, permutations: Vec<Vec<NodeIndex>>, number_of_workers: u32) -> u32 {
        let total_possibilities = permutations.len() * permutations.len();
        let mut computed_possibilities: usize = 0;
        let mut percent_computed_possibilities: usize = computed_possibilities / total_possibilities;
        let start = SystemTime::now();
        println!("Number of possibilities: {}", total_possibilities);
        let mut best = 0;
        let (snd1, rcv1) = crossbeam_channel::bounded(1);
        let (snd2, rcv2) = crossbeam_channel::bounded(1);

        crossbeam::scope(|scope| {
            // Producer thread
            scope.spawn(|_| {
                for player_path in &permutations {
                    for elephant_path in &permutations {
                        snd1.send((player_path, elephant_path)).unwrap();
                    }
                }
                // Close the channel - this is necessary to exit
                // the for-loop in the worker
                drop(snd1);
            });

            // Parallel processing
            for _ in 0..number_of_workers {
                // send to sink, receive from source
                let (sendr, recvr) = (snd2.clone(), rcv1.clone());
                // spawn worker in separate threads
                scope.spawn(move |_| {
                    for message in recvr.iter() {
                        let val = self.walk_routine_with_elephant(message);
                        sendr.send(val).unwrap();
                    }
                });
            }

            // Close the channel, otherwise sink will never
            // exit the for-loop
            drop(snd2);

            // Sink
            for msg in rcv2.iter() {
                best = max(msg, best);
                computed_possibilities += 100;
                let temp_percent_computed_possibilities = computed_possibilities / total_possibilities;
                if percent_computed_possibilities < temp_percent_computed_possibilities {
                    percent_computed_possibilities = temp_percent_computed_possibilities;
                    let end = SystemTime::now();
                    let duration = end.duration_since(start).expect("Time went backwards.");
                    println!("Progress: {}%   Duration: {:?}", percent_computed_possibilities, duration);
                }
            }
        }).unwrap();

        best
    }

    fn walk_routine_with_elephant(&self, (path_1, path_2): (&Vec<NodeIndex>, &Vec<NodeIndex>)) -> u32 {
        let mut pressure_released = 0;
        let mut minutes_left_1: u32 = self.minutes_left;
        let mut minutes_left_2: u32 = self.minutes_left;

        let mut path_1_iter = path_1.iter();
        let mut path_2_iter = path_2.iter();

        let mut nodes_history_vec_1 = Vec::new();
        let mut nodes_history_vec_2 = Vec::new();

        nodes_history_vec_1.push(path_1_iter.next().unwrap());
        nodes_history_vec_2.push(path_2_iter.next().unwrap());

        let mut visited = HashMap::new();

        loop {
            if minutes_left_2 <= minutes_left_1 {
                match path_1_iter.next() {
                    Some(node_index_1) => {
                        let (m_left, p_released) = self.visit_node(&mut visited, &mut nodes_history_vec_1, node_index_1, minutes_left_1);
                        minutes_left_1 = m_left;
                        pressure_released += p_released;
                    }
                    None => {
                        match path_2_iter.next() {
                            Some(node_index_2) => {
                                let (m_left, p_released) = self.visit_node(&mut visited, &mut nodes_history_vec_2, node_index_2, minutes_left_2);
                                minutes_left_2 = m_left;
                                pressure_released += p_released;
                            }
                            None => break
                        }
                    }
                }
            } else {
                match path_2_iter.next() {
                    Some(node_index_2) => {
                        let (m_left, p_released) = self.visit_node(&mut visited, &mut nodes_history_vec_2, node_index_2, minutes_left_2);
                        minutes_left_2 = m_left;
                        pressure_released += p_released;
                    }
                    None => {
                        match path_1_iter.next() {
                            Some(node_index_1) => {
                                let (m_left, p_released) = self.visit_node(&mut visited, &mut nodes_history_vec_1, node_index_1, minutes_left_1);
                                minutes_left_1 = m_left;
                                pressure_released += p_released;
                            }
                            None => break
                        }
                    }
                }
            }
        }

        pressure_released
    }

    fn visit_node<'a>(&self, visited: &mut HashMap<&'a NodeIndex, bool>, nodes_history_vec: &mut Vec<&'a NodeIndex>, node_index: &'a NodeIndex, mut minutes_left: u32) -> (u32, u32) {
        let mut pressure_released = 0;
        if !visited.contains_key(&node_index) {
            visited.insert(node_index, true);
            minutes_left -= self.path_map.get(&(**nodes_history_vec.last().unwrap(), *node_index)).unwrap() + 1;
            pressure_released += self.graph.nodes[*node_index].flow_rate * minutes_left;
        } else {
            minutes_left -= self.path_map.get(&(**nodes_history_vec.last().unwrap(), *node_index)).unwrap();
        }
        nodes_history_vec.push(node_index);
        (minutes_left, pressure_released)
    }
}

pub fn read_in_file_system(file_path: &str) -> Result<HashMap<String, (u32, Vec<String>)>, io::Error> {
    let lines = read_lines(file_path)?;
    // Valve GS has flow rate=0; tunnels lead to valves KB, GW
    let line_regex = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    let mut volcano_map = HashMap::new();
    for line in lines {
        if let Ok(l) = line{
            if let Some(cap) = line_regex.captures_iter(&l).next() {
                let valve_name: String = cap[1].parse().unwrap();
                let flow_rate: u32 = cap[2].parse().unwrap();
                let connected_valves_string: String = cap[3].parse().unwrap();
                if connected_valves_string.contains(", ") {
                    let connected_valves: Vec<String> = connected_valves_string.split(", ").map(|x| String::from(x)).collect();
                    volcano_map.insert(valve_name, (flow_rate, connected_valves));
                } else {
                    volcano_map.insert(valve_name, (flow_rate, vec![connected_valves_string]));
                }
            }
        }
    }
    Ok(volcano_map)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{read_in_file_system, Volcano};

    #[test]
    fn test_read_in() {
        let volcano_map = read_in_file_system("input_test");
        assert!(volcano_map.is_ok())
    }

    #[test]
    fn test_volcano_creation() {
        let volcano_map = read_in_file_system("input_test").unwrap();
        let volcano = Volcano::new(volcano_map, 30);
        assert!(volcano.is_ok())
    }

    #[test]
    fn test_puzzle_01() {
        let volcano_map = read_in_file_system("input_test").unwrap();
        let mut volcano = Volcano::new(volcano_map, 30).unwrap();
        assert_eq!(volcano.brute_force_path_search(), 1651)
    }

    #[test]
    fn test_puzzle_02() {
        let volcano_map = read_in_file_system("input_test").unwrap();
        let mut volcano = Volcano::new(volcano_map, 26).unwrap();
        assert_eq!(volcano.brute_force_path_search_with_elephant(), 1707)
    }

}