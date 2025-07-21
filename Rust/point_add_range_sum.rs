use std::io;

#[derive(Debug)]
struct Node {
    value: i64,

    // this is a [l, r) range.
    left_range: usize,
    right_range: usize,

    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

impl Node {
    fn create(l: usize, r: usize) -> Option<Box<Node>> {
        if l >= r {
            return None;
        }
        
        if l + 1 == r {
            return Some(Box::new(Node {
                value: 0, 
                left_range: l,
                right_range: r,
                left_node: None,
                right_node: None
            }))
        }
        
        let mid: usize = (l + r)/2;

        Some(Box::new(Node {
            value: 0, 
            left_range: l,
            right_range: r,
            left_node: Node::create(l , mid),
            right_node: Node::create(mid, r)
        }))
    }

    fn new(l: usize, r: usize) -> Option<Box<Node>> {
        Node::create(l, r)
    }
}

#[derive(Debug)]
pub struct SegmentTree {
    root: Option<Box<Node>>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self { root: Node::new(0, n) }
    }

    fn set_val(node: Option<&mut Box<Node>>, index: usize, val: i64) {
        if let Some(node) = node {
            if index < node.left_range || index >= node.right_range {
                return;
            }

            if index == node.left_range && index + 1 == node.right_range {
                node.value = val;
            } else {
                SegmentTree::set_val(node.left_node.as_mut(), index, val);
                SegmentTree::set_val(node.right_node.as_mut(), index, val);

                let left_val = node.left_node.as_ref().map_or(0, |n| n.value);
                let right_val = node.right_node.as_ref().map_or(0, |n| n.value);
                node.value = left_val + right_val;
            }
        }
    }

    fn set(&mut self, index: usize, val: i64) {
        SegmentTree::set_val(self.root.as_mut(), index, val)
    }

    fn get_sum(node: Option<&Box<Node>>, l: usize, r: usize) -> i64 {
        if let Some(node) = node {
            if r < node.left_range || l > node.right_range {
                return 0;
            }

            if l <= node.left_range && node.right_range <= r {
                return node.value;
            } else {
                SegmentTree::get_sum(node.left_node.as_ref(), l, r) + SegmentTree::get_sum(node.right_node.as_ref(), l, r)
            }
        } else {
            0
        }
    }

    fn get(&self, l: usize, r: usize) -> i64 {
        SegmentTree::get_sum(self.root.as_ref(), l, r)
    }
}

#[derive(Debug)]
enum Query {
    Set { p: usize, x: i64 },
    Get { l: usize, r: usize }
}

fn read_two_i32() -> (usize, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn read_vector_i64(n: &usize) -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let mut vec = Vec::<i64>::new();
    for i in 0..*n as i32 {
        vec.push(iter.next().unwrap().parse().unwrap());
    }
    vec
}

fn read_query() -> Query {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let t: i32 = iter.next().unwrap().parse().unwrap();
    let u: usize = iter.next().unwrap().parse().unwrap();
    let v: usize = iter.next().unwrap().parse().unwrap();


    match t {
        0 => Query::Set {p: u, x: v as i64},
        1 => Query::Get {l: u, r: v},
        _ => unreachable!()
    }
}

fn main() {
    let (n, mut q) = read_two_i32();
    let mut st = SegmentTree::new(n);

    let vec = read_vector_i64(&n);

    for (i, v) in vec.into_iter().enumerate() {
        st.set(i, v);
    }

    while { let tmp = q; q -= 1; tmp } > 0 {
        let q = read_query();

        match q {
            Query::Set {p, x} => {st.set(p, x + st.get(p, p + 1)); },
            Query::Get {l, r} => {println!("{}", st.get(l, r));}
        }
    }
}