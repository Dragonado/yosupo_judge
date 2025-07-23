use std::collections::HashMap;
use std::io::{self, BufRead};

/// Represents a directed graph using an adjacency list.
#[derive(Debug)]
struct Graph {
    /// `adj[i]` contains a list of pairs `(neighbor, edge_index)` for node `i`.
    adj: Vec<Vec<(usize, usize)>>,
    size: usize,
}

impl Graph {
    /// Creates a new Graph from a given size and a list of edges.
    pub fn new(size: usize, edges: &[(usize, usize, usize)]) -> Self {
        let mut adj = vec![Vec::new(); size];

        for &(u, v, i) in edges {
            adj[u].push((v, i));
        }

        Self { adj, size }
    }

    /// Searches for any cycle in the graph.
    ///
    /// Returns `Some(Vec<usize>)` containing the edge labels of a path that
    /// ends in a cycle, or `None` if the graph is a Directed Acyclic Graph (DAG).
    pub fn get_cycle(&self) -> Option<Vec<usize>> {
        let mut is_visited = vec![false; self.size];
        let mut recursion_stack = HashMap::new();
        let mut path_labels = Vec::new();

        // Iterate through all nodes to handle disconnected graphs.
        for i in 0..self.size {
            if !is_visited[i] {
                // Start a new DFS from this unvisited node.
                recursion_stack.insert(i, 1);
                if self.find_cycle_recursive(i, &mut is_visited, &mut recursion_stack, &mut path_labels) {
                    return Some(path_labels);
                }
                // Backtrack for the starting node of the DFS tree.
                recursion_stack.remove(&i);
            }
        }
        None
    }

    /// Performs a Depth First Search (DFS) to find a cycle.
    /// This is a recursive helper function for `get_cycle`.
    fn find_cycle_recursive(
        &self,
        current_node: usize,
        is_visited: &mut [bool],
        // Tracks nodes in the current recursion path to detect back edges.
        recursion_stack: &mut HashMap<usize, usize>,
        path_labels: &mut Vec<usize>,
    ) -> bool {
        is_visited[current_node] = true;

        for &(neighbor, edge_index) in &self.adj[current_node] {
            // If the neighbor is already in the current recursion stack, we found a back edge.
            if recursion_stack.get(&neighbor).unwrap_or(&0) > &0 {
                path_labels.push(edge_index);
                return true; // Cycle detected!
            }

            // If the neighbor was visited in a *previous* DFS tree, skip it.
            if is_visited[neighbor] {
                continue;
            }

            // Add node to stack and path before descending.
            *recursion_stack.entry(neighbor).or_insert(0) += 1;
            path_labels.push(edge_index);

            if self.find_cycle_recursive(neighbor, is_visited, recursion_stack, path_labels) {
                return true; // Propagate the "found" signal up the call stack.
            }

            // Backtrack: remove node from stack and path.
            *recursion_stack.get_mut(&neighbor).unwrap() -= 1;
            path_labels.pop();
        }

        false // No cycle found from this node.
    }
}

/// Trims the "tail" from a path that ends in a cycle.
///
/// The path from `get_cycle` is like `A -> B -> ... -> X -> Y -> ... -> Z -> X`,
/// where the cycle starts at node `X`. This function removes the `A -> B -> ...` tail
/// and returns only the edges that form the actual cycle.
fn remove_tail(labels: Vec<usize>, edges: &[(usize, usize, usize)]) -> Vec<usize> {
    if labels.is_empty() {
        return Vec::new();
    }

    // Create a lookup map from an edge's index to its nodes for quick access.
    let edge_map: HashMap<usize, (usize, usize)> =
        edges.iter().map(|&(u, v, i)| (i, (u, v))).collect();

    // Reconstruct the path of nodes from the edge labels.
    // e.g., for edges [e(A,B), e(B,C), e(C,B)], the node path is [A, B, C, B].
    let mut node_path = Vec::new();
    if let Some(&(first_u, _)) = edge_map.get(&labels[0]) {
        node_path.push(first_u); // Start with the source of the first edge.
    }
    for &label in &labels {
        if let Some(&(_, v)) = edge_map.get(&label) {
            node_path.push(v); // Add the destination of each subsequent edge.
        }
    }

    // Find the first node that repeats. Its initial position marks the cycle's start.
    // `seen_nodes` maps a node to the index of its first appearance in the path.
    let mut seen_nodes = HashMap::new();
    let mut cycle_start_index = 0;

    for (i, &node) in node_path.iter().enumerate() {
        if let Some(&first_pos) = seen_nodes.get(&node) {
            // This node has been seen before. The cycle starts at the edge
            // corresponding to its first appearance.
            cycle_start_index = first_pos;
            break;
        }
        // `node_path[i]` is the start node for the edge at `labels[i]`.
        seen_nodes.insert(node, i);
    }

    // The cycle is the slice of the labels vector from the calculated start index.
    labels[cycle_start_index..].to_vec()
}

fn main() {
    let stdin = io::stdin();
    // Lock stdin for faster I/O and get an iterator over lines.
    let mut lines = stdin.lock().lines();

    // --- Input Processing ---
    let first_line = lines.next().unwrap().expect("Failed to read the first line");
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = parts.next().unwrap().parse().expect("Failed to parse m");

    // Read the m edges and assign them an index based on input order.
    // Using `map().collect()` is an idiomatic way to build the vector.
    let edges: Vec<(usize, usize, usize)> = (0..m)
        .map(|i| {
            let line = lines.next().unwrap().expect("Failed to read an edge line");
            let mut parts = line.split_whitespace();
            let u: usize = parts.next().unwrap().parse().expect("Failed to parse u");
            let v: usize = parts.next().unwrap().parse().expect("Failed to parse v");
            (u, v, i)
        })
        .collect();

    // --- Cycle Finding and Output ---
    let g = Graph::new(n, &edges);
    if let Some(path_with_cycle) = g.get_cycle() {
        let cycle_labels = remove_tail(path_with_cycle, &edges);

        if cycle_labels.len() > 1 {
            println!("{}", cycle_labels.len());
            for label in cycle_labels {
                println!("{}", label);
            }
        } else {
             unreachable!();
        }
    } else {
        println!("-1");
    }
}