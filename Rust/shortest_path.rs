use std::io::{self, BufRead};
use std::collections::BinaryHeap;
use std::cmp::Ordering;


/// Represents a directed graph using an adjacency list.
#[derive(Debug)]
struct Graph {
    /// `adj[i]` contains a list of pairs `(neighbor, weight)` for node `i`.
    adj: Vec<Vec<(usize, i64)>>,
    prev_node: Vec<usize>,
    source_node: usize,
    shortest_path_vec: Vec<i64>,
}

#[derive(Eq, PartialEq, Debug)]
struct DijState{
    node: usize,
    distance: i64,
}

impl Ord for DijState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for DijState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Some(other.distance.cmp(&self.distance))
        Some(self.cmp(other))
    }
}

impl Graph {
    /// Creates a new Graph from a given size and a list of edges.
    pub fn new(size: usize, edges: &[(usize, usize, i64)], source_node: usize) -> Self {
        let mut adj = vec![Vec::new(); size];
        let prev_node = vec![usize::MAX; size];
        let shortest_path_vec = vec![i64::MAX; size];
        for &(u, v, w) in edges {
            adj[u].push((v, w));
        }

        Self { adj, prev_node, source_node, shortest_path_vec}
    }

    pub fn get_shortest_path(&self, v: usize) -> Option<(i64, usize, Vec<(usize, usize)>)> {
        let distance = self.shortest_path_vec[v]; 
        if distance == i64::MAX {
            return None
        }
        
        let mut ans = Vec::<(usize, usize)>::new();
        let mut node = v;

        while node != self.source_node {
            ans.push((self.prev_node[node], node));
            node = self.prev_node[node];
        }
        
        ans.reverse();

        Some((distance, ans.len(), ans))
    }

    pub fn populate_all_shortest_path(&mut self) {
        let mut pq = BinaryHeap::new();
        self.shortest_path_vec[self.source_node] = 0;
        self.prev_node[self.source_node] = self.source_node;

        pq.push(DijState {node: self.source_node, distance: 0 });

        while !pq.is_empty() {
            if let Some(s) = pq.pop() {
                if s.distance > self.shortest_path_vec[s.node] {
                    continue;
                }
                for (neighbour, weight) in &self.adj[s.node] {
                    if self.shortest_path_vec[*neighbour] > s.distance + *weight {
                        self.shortest_path_vec[*neighbour] = s.distance + *weight;
                        self.prev_node[*neighbour] = s.node;
                        pq.push(DijState {node: *neighbour, distance: self.shortest_path_vec[*neighbour]});
                    }
                }
            }
        }
    }
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
    let s: usize = parts.next().unwrap().parse().expect("Failed to parse s");
    let t: usize = parts.next().unwrap().parse().expect("Failed to parse t");

    // Read the m edges and assign them an index based on input order.
    // Using `map().collect()` is an idiomatic way to build the vector.
    let edges: Vec<(usize, usize, i64)> = (0..m)
        .map(|_| {
            let line = lines.next().unwrap().expect("Failed to read an edge line");
            let mut parts = line.split_whitespace();
            let u: usize = parts.next().unwrap().parse().expect("Failed to parse u");
            let v: usize = parts.next().unwrap().parse().expect("Failed to parse v");
            let c: i64 = parts.next().unwrap().parse().expect("Failed to parse c");
            (u, v, c)
        })
        .collect();
    
    let mut g = Graph::new(n, &edges, s);
    g.populate_all_shortest_path();

    match g.get_shortest_path(t) { 
        None => println!("-1"),
        Some((distance, len, ans)) => {
            println!("{} {}", distance, len);
            for (u, v) in ans {
                println!("{} {}", u, v);
            }
        }
    }
}