use std::io::{self, BufRead};
use std::collections::HashMap;

/// Represents a single node in the segment tree.
/// Using std::ops::Range makes the [start, end) interval explicit and provides useful methods.
#[derive(Debug)]
struct Graph {
    adj: Vec<Vec<(usize, usize)>>,
    size: usize
}

impl Graph {
    /// Creates a new SegmentTree for a sequence of `size` elements.
    pub fn new(size: &usize, edges: &Vec<(usize, usize, usize)>) -> Self {
        let mut adj = vec![Vec::new(); *size];

        // dbg!(&size, &edges);
        for (u, v, i) in edges {
            adj[*u].push((*v, *i));
        }

        Self {
            adj,
            size: *size,
        }
    }

    fn find_cycle(&self, cur_node: usize, visited: &mut Vec<bool>, right_node_cnt: &mut HashMap<usize, usize>, labels: &mut Vec<usize>) -> bool {
        // dbg!(&labels);
        visited[cur_node] = true;

        for (ne, index) in &self.adj[cur_node] {
            let ne = *ne;
            let index = *index;

            if let Some(cnt) = right_node_cnt.get(&ne) {
                // dbg!(ne, &cnt)÷;
                if *cnt > 0 {
                    labels.push(index);
                    return true;
                }
            }

            // visited by someone else.
            if visited[ne] == true {
                continue;
            }

            right_node_cnt.entry(ne.clone()).and_modify(|cnt| *cnt += 1).or_insert(1);
            labels.push(index);

            if self.find_cycle(ne, visited, right_node_cnt, labels){
                return true;
            }

            right_node_cnt.entry(ne.clone()).and_modify(|cnt| *cnt -= 1).or_insert(1);
            labels.pop();
        }

        false
    }

    pub fn get_cycle(&self) -> Option<Vec<usize>> {
        let mut visited = vec![false; self.size];
        let mut right_node_cnt = HashMap::new();
        let mut labels = Vec::new();

        for i in 0..self.size {
            if visited[i] == true {
                continue;
            }
            
            right_node_cnt.entry(i.clone()).and_modify(|cnt| *cnt += 1).or_insert(1);
            if self.find_cycle(i, &mut visited,  &mut right_node_cnt, &mut labels)  {
                return Some(labels);
            }
            right_node_cnt.entry(i.clone()).and_modify(|cnt| *cnt -= 1).or_insert(1);
        }
        None
    }
}

fn remove_tail(labels: Vec<usize>, edges: Vec<(usize, usize, usize)>) -> Vec<usize> {
    let mut edge_map = HashMap::<usize, (usize, usize)>::new();
    for (u, v, i) in &edges {
        edge_map.insert(*i, (*u, *v));
    }

    let mut right_node_cnt = HashMap::new();

    for i in &labels {
        let (_, v) = edge_map[i];

        right_node_cnt.entry(v.clone()).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    let mut index:usize = 0;
    for i in &labels { 
        let (_, v) = edge_map[i];

        if let Some(cnt) = right_node_cnt.get(&v) { 
            if *cnt > 1 {
                index = i+1;
                break;
            }
        }
    }

    // dbg!(&index, &labels);
    let mut ans = Vec::new();
    for j in index..labels.len() {
        ans.push(labels[j]);
    }

    // dbg!(&ans);
    ans
}


fn main() {
    // Use a buffered reader for more efficient I/O from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    // Read n and m from the first line.
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Failed to parse n");
    let m: usize = parts.next().unwrap().parse().expect("Failed to parse m");

    
    let mut edges = Vec::<(usize, usize, usize)>::new();

    for i in 0_usize..m {
        let edge_lines = lines.next().unwrap();
        let mut parts = edge_lines.split_whitespace();
        let u: usize = parts.next().unwrap().parse().expect("Failed to parse u");
        let v: usize = parts.next().unwrap().parse().expect("Failed to parse v");

        edges.push((u, v, i));

    }
    
    let g = Graph::new(&n, &edges);

    if let Some(labels) = g.get_cycle() { 
        let labels = remove_tail(labels, edges);

        assert!(labels.len() > 1);

        println!("{}", labels.len());
        for label in labels {
            println!("{}", label);
        }
    }else {
        println!("-1");
    }
}