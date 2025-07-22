use std::io::{self, BufRead};
use std::ops::Range;

/// Represents a single node in the segment tree.
/// Using std::ops::Range makes the [start, end) interval explicit and provides useful methods.
#[derive(Debug)]
struct Node<T: Clone> {
    value: T,
    range: Range<usize>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    /// Creates a new node and recursively builds its children to cover the given range.
    fn new(range: Range<usize>, e: &impl Fn() -> T) -> Option<Box<Node<T>>> {
        // An empty range results in no node.
        if range.is_empty() {
            return None;
        }

        let mut node = Box::new(Node {
            value: e(),
            range: range.clone(),
            left: None,
            right: None,
        });

        // If the range represents more than one element, it's an internal node, so create children.
        if range.len() > 1 {
            let mid = range.start + range.len() / 2;
            node.left = Node::new(range.start..mid, e);
            node.right = Node::new(mid..range.end, e);
        }

        Some(node)
    }

    /// Recalculates this node's value based on its children's values.
    /// This is called after a child's value has been updated.
    fn update_value(&mut self, e: &impl Fn() -> T, op: &impl Fn(T, T) -> T) {
        let left_val = self.left.as_ref().map_or(e(), |n| n.value.clone());
        let right_val = self.right.as_ref().map_or(e(), |n| n.value.clone());
        self.value = op(left_val, right_val);
    }
}

/// A segment tree implementation for sum queries on a range.
#[derive(Debug)]
pub struct SegmentTree<T, E, OP>
where
    T: Clone,
    E: Fn() -> T,
    OP: Fn(T, T) -> T,
{
    root: Option<Box<Node<T>>>,
    size: usize,
    e: E, // function that returns the identity element
    op: OP, // function that combines two elements of T and gives the result.
}

impl<T, E, OP> SegmentTree<T, E, OP>
where
    T: Clone,
    E: Fn() -> T + Clone,
    OP: Fn(T, T) -> T + Clone,
{
    /// Creates a new SegmentTree for a sequence of `size` elements.
    pub fn new(size: usize, e: E, op: OP) -> Self {
        Self {
            root: Node::new(0..size, &e),
            size,
            e,
            op,
        }
    }

    /// Sets the value at a specific index.
    pub fn set(&mut self, index: usize, val: T) {
        // Ensure the index is within the bounds of the tree.
        if index >= self.size {
            return; // Or handle with panic!/Result as needed.
        }
        if let Some(root) = self.root.as_mut() {
            Self::set_recursive(root, index, val, &self.e, &self.op);
        }
    }

    /// Helper function to recursively find the correct leaf node and update values up the tree.
    fn set_recursive(node: &mut Node<T>, index: usize, val: T, e: &E, op: &OP) {
        // Base case: we have reached the leaf node corresponding to the index.
        if node.range.len() == 1 {
            node.value = val;
            return;
        }

        // Recursive step: determine whether to go left or right.
        let mid = node.range.start + node.range.len() / 2;
        // The `unwrap`s here are safe due to the invariant that non-leaf nodes always have children.
        if index < mid {
            Self::set_recursive(node.left.as_mut().unwrap(), index, val, e, op);
        } else {
            Self::set_recursive(node.right.as_mut().unwrap(), index, val, e, op);
        }

        // After recursion, update the current node's value based on its children.
        node.update_value(e, op);
    }

    /// Returns the sum of values in the given half-open range `[start, end)`.
    pub fn get(&self, query_range: Range<usize>) -> T {
        self.root
            .as_ref()
            .map_or((self.e)(), |root| Self::get_recursive(root, &query_range, &self.e, &self.op))
    }

    /// Helper function to recursively calculate the sum over a given query range.
    fn get_recursive(node: &Node<T>, query_range: &Range<usize>, e: &E, op: &OP) -> T {
        // Case 1: The node's range has no overlap with the query range.
        if query_range.end <= node.range.start || query_range.start >= node.range.end {
            return e();
        }

        // Case 2: The node's range is completely contained within the query range.
        if query_range.start <= node.range.start && query_range.end >= node.range.end {
            return node.value.clone();
        }

        // Case 3: Partial overlap. Recurse into children and sum their results.
        let left_sum = node
            .left
            .as_ref()
            .map_or(e(), |n| Self::get_recursive(n, query_range, e, op));
        let right_sum = node
            .right
            .as_ref()
            .map_or(e(), |n| Self::get_recursive(n, query_range, e, op));

        op(left_sum, right_sum)
    }
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

    // Example: sum segment tree over i64
    let mut st = SegmentTree::<i64, _, _>::new(n, || i64::MAX, |a, b| std::cmp::min(a, b));

    // Read initial array values and populate the segment tree.
    if n > 0 {
        let initial_values: Vec<i64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse initial value"))
            .collect();

        for (i, &v) in initial_values.iter().enumerate() {
            st.set(i, v);
        }
    }

    // Process q queries.
    for _ in 0..q {
        let query_line = lines.next().unwrap();
        let mut parts = query_line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().expect("Failed to parse l");
        let r: usize = parts.next().unwrap().parse().expect("Failed to parse r");

        println!("{}", st.get(l..r));
    }
}
