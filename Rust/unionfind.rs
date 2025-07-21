use std::io;
use std::mem::swap;

#[derive(Debug)]
struct UnionFind{
    parent: Vec<usize>,
    height: Vec<usize>
}

impl UnionFind {
    fn new(n: &usize) -> Self {
        let mut parent = Vec::<usize>::new();
        let mut height = Vec::<usize>::new();
        for i in 0..*n {
            parent.push(i);
            height.push(1);
        }
        Self {parent, height}
    }

    fn get_parent(&mut self, u: usize) -> usize {
        match self.parent[u] == u {
            true => u,
            false => {
                // path compression.
                self.parent[u] = self.get_parent(self.parent[u]);
                self.parent[u]
            }
        }
    }

    fn merge(&mut self, u: usize, v: usize) {
        let mut u = self.get_parent(u);
        let mut v = self.get_parent(v);

        if u != v {
            // balance on height.
            if self.height[u] > self.height[v] {
                swap(&mut u, &mut v);
            }

            self.parent[u] = v;

            if self.height[u] == self.height[v] {
                self.height[v] += 1;
            }
        }
    }
}

#[derive(Debug)]
enum Query {
    Set { u: usize, v: usize },
    Get { u: usize, v: usize }
}

fn read_two_i32() -> (usize, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn read_query() -> Query {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let t: i32 = iter.next().unwrap().parse().unwrap();
    let u: usize = iter.next().unwrap().parse().unwrap();
    let v: usize = iter.next().unwrap().parse().unwrap();


    match t {
        0 => Query::Set {u, v },
        1 => Query::Get {u, v},
        _ => unreachable!()
    }
}

fn main() {
    let (n, mut q) = read_two_i32();
    let mut uf = UnionFind::new(&n);

    while { let tmp = q; q -= 1; tmp } > 0 {
        let q = read_query();
        match q {
            Query::Get { u, v } => {println!("{}", (uf.get_parent(u) == uf.get_parent(v)) as i32);},
            Query::Set {u, v} => uf.merge(u, v)
        }
    }
}