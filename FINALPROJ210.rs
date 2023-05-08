use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::data::FromElements;

fn main() {
    // Open the file
    let file = File::open("winequality-red.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    let mut graph = petgraph::Graph::<(), ()>::new();
    // Parse the file + add nodes and edges
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.trim().split(" ").collect();
        let src_node = parts[0].parse::<usize>().expect("Failed to parse source node ID");
        let dst_node = parts[1].parse::<usize>().expect("Failed to parse destination node ID");
        graph.add_node(());
        graph.add_node(());
        graph.add_edge(src_node, dst_node, ());
    }
    
}

use std::collections::{HashSet, VecDeque};

pub fn bfs<'a, T>(
    graph: &'a HashMap<&'a T, HashSet<&'a T>>,
    start: &'a T,
) -> HashMap<&'a T, usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    visited.insert(start);
    queue.push_back(start);
    distances.insert(start, 0);

    while let Some(node) = queue.pop_front() {
        let distance = distances[node];
        for neighbor in graph.get(node).unwrap().iter() {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
                distances.insert(neighbor, distance + 1);
            }
        }
    }

    distances
}

pub fn compute_distances<'a, T>(graph: &'a HashMap<&'a T, HashSet<&'a T>>) -> u64 {
    let mut total_distance = 0;
    let mut num_pairs = 0;
    let nodes = graph.keys().collect::<Vec<_>>();
    for i in 0..nodes.len() 
        let start = nodes
}
fn get_distance(graph: &Graph, start: &str, end: &str) -> Option<u32> 
    let start_node = match graph.nodes.get(start) {
        Some(node) => node,
        None => return None,
    };
        
    let end_node = match graph.nodes.get(end) {
        Some(node) => node,
        None => return None,
    };

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_node, 0));

    while let Some((node, distance)) = queue.pop_front() {
        if visited.contains(node) {
            continue;
        }

        if node == end_node {
            return Some(distance);
        }

        visited.insert(node);
        for neighbor in node.neighbors.iter() {
            queue.push_back((neighbor, distance + 1));
        }
    }

    None
}

fn main() {
    let graph = Graph::new();

    let distance = get_distance(&graph, "A", "G");
    match distance {
        Some(d) => println!("Distance between A and G: {}", d),
        None => println!("No path found between A and G"),
    }
}
