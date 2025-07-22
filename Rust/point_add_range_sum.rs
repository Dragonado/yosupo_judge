use std::io::{self, BufRead};
use std::ops::Range;

pub trait Monoid {
    // Required methods
    fn id() -> Self;
    fn op(a: &Self, b: &Self) -> Self;
}

/// Represents a single node in the segment tree.
/// Using std::ops::Range makes the [start, end) interval explicit and provides useful methods.
#[derive(Debug)]
struct Node<T: Monoid + Clone> {
    value: T,
    range: Range<usize>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Monoid + Clone> Node<T> {
    /// Creates a new node and recursively builds its children to cover the given range.
    fn new(range: Range<usize>) -> Option<Box<Node<T>>> {
        // An empty range results in no node.
        if range.is_empty() {
            return None;
        }

        let mut node = Box::new(Node {
            value: T::id(),
            range: range.clone(),
            left: None,
            right: None,
        });

        // If the range represents more than one element, it's an internal node, so create children.
        if range.len() > 1 {
            let mid = range.start + range.len() / 2;
            node.left = Node::new(range.start..mid);
            node.right = Node::new(mid..range.end);
        }

        Some(node)
    }

    /// Recalculates this node's value based on its children's values.
    /// This is called after a child's value has been updated.
    fn update_value(&mut self) {
        let left_val = self.left.as_ref().map_or(T::id(), |n| n.value.clone());
        let right_val = self.right.as_ref().map_or(T::id(), |n| n.value.clone());
        self.value = T::op(&left_val, &right_val);
    }
}

/// A segment tree implementation for sum queries on a range.
#[derive(Debug)]
pub struct SegmentTree<T>
where
    T: Monoid + Clone
{
    root: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> SegmentTree<T>
where
    T: Monoid + Clone,
{
    /// Creates a new SegmentTree for a sequence of `size` elements.
    pub fn new(size: usize) -> Self {
        Self {
            root: Node::new(0..size),
            size,
        }
    }

    /// Sets the value at a specific index.
    pub fn set(&mut self, index: usize, val: T) {
        // Ensure the index is within the bounds of the tree.
        if index >= self.size {
            return;
        }
        if let Some(root) = self.root.as_mut() {
            Self::set_recursive(root, index, val);
        }
    }

    /// Helper function to recursively find the correct leaf node and update values up the tree.
    fn set_recursive(node: &mut Node<T>, index: usize, val: T) {
        // Base case: we have reached the leaf node corresponding to the index.
        if node.range.len() == 1 {
            node.value = val;
            return;
        }

        // Recursive step: determine whether to go left or right.
        let mid = node.range.start + node.range.len() / 2;
        // The `unwrap`s here are safe due to the invariant that non-leaf nodes always have children.
        if index < mid {
            Self::set_recursive(node.left.as_mut().unwrap(), index, val);
        } else {
            Self::set_recursive(node.right.as_mut().unwrap(), index, val);
        }

        // After recursion, update the current node's value based on its children.
        node.update_value();
    }

    /// Returns the sum of values in the given half-open range `[start, end)`.
    pub fn get(&self, query_range: Range<usize>) -> T {
        self.root
            .as_ref()
            .map_or(T::id(), |root| Self::get_recursive(root, &query_range))
    }

    /// Helper function to recursively calculate the sum over a given query range.
    fn get_recursive(node: &Node<T>, query_range: &Range<usize>) -> T {
        // Case 1: The node's range has no overlap with the query range.
        if query_range.end <= node.range.start || query_range.start >= node.range.end {
            return T::id();
        }

        // Case 2: The node's range is completely contained within the query range.
        if query_range.start <= node.range.start && query_range.end >= node.range.end {
            return node.value.clone();
        }

        // Case 3: Partial overlap. Recurse into children and sum their results.
        let left_sum = node
            .left
            .as_ref()
            .map_or(T::id(), |n| Self::get_recursive(n, query_range));
        let right_sum = node
            .right
            .as_ref()
            .map_or(T::id(), |n| Self::get_recursive(n, query_range));

        T::op(&left_sum, &right_sum)
    }
}


#[derive(Clone)]
struct S {
    val: i64
}
impl Monoid for S {
    fn id() -> Self { S {val: 0 } }
    fn op(a: &Self, b: &Self) -> Self { S {val: a.val + b.val } }
}

fn main() {
    // Use a buffered reader for more efficient I/O from stdin.
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    // Read n and q from the first line.
    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Failed to parse n");
    let q: usize = parts.next().unwrap().parse().expect("Failed to parse q");

    
    let mut st = SegmentTree::<S>::new(n);

    // Read initial array values and populate the segment tree.
    if n > 0 {
        let initial_values: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse initial value"))
            .collect();

        for (i, &v) in initial_values.iter().enumerate() {
            st.set(i, S {val: v as i64});
        }
    }

    // Process q queries.
    for _ in 0..q {
        let query_line = lines.next().unwrap();
        let mut parts = query_line.split_whitespace();
        let t: usize = parts.next().unwrap().parse().expect("Failed to parse l");
        let p: usize = parts.next().unwrap().parse().expect("Failed to parse l");
       
        
        match t{
            0 => {
                let x: i64 = parts.next().unwrap().parse().expect("Failed to parse r");
                st.set(p, S::op(&S{val: x}, &st.get(p..p+1)));
            }
            1 => {
                let l = p;
                let r: usize = parts.next().unwrap().parse().expect("Failed to parse r");
                println!("{}", st.get(l..r).val);
            }
            _ => unreachable!()
        }
    }
}
