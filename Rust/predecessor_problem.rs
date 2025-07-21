// This problem can be solved better once btree_cursors are implemented.
// https://github.com/rust-lang/rust/issues/107540 
// Having an object point to a gap between two items is just next level and makes the implementation for this problem way easier.
use std::io;
use std::collections::BTreeSet;

#[derive(Debug)]
enum Query {
    Insert {k: i64},
    Remove {k: i64}, 
    Exists {k: i64},
    Next {k: i64},
    Previous {k: i64}
}

fn read_two_i64() -> (i64, i64) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().unwrap();
    let b: i64 = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn read_query() -> Query {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let t: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    match t {
        0 => Query::Insert {k},
        1 => Query::Remove {k},
        2 => Query::Exists {k},
        3 => Query::Previous {k},
        4 => Query::Next {k},
        _ => unreachable!()
    }
}

fn main() -> io::Result<()> {
    let (_n, mut q) = read_two_i64();

    let mut initial_state = String::new();
     io::stdin().read_line(&mut initial_state)?;
    // dbg!(&initial_state);

    let mut set = BTreeSet::<i64>::new();

    for (i, c) in initial_state.trim().chars().enumerate() {
        match c {
            '0' => (),
            '1' => { set.insert(i as i64); }
            _ => unreachable!(),
        }
    }

    while { let tmp = q; q -= 1; tmp } > 0 {
        let query = read_query();
        match query {
            Query::Insert {k} => {set.insert(k);},
            Query::Remove {k} => {set.remove(&k);},
            Query::Exists {k} => {println!("{}", set.contains(&k) as i32);},
            Query::Next {k} => {
                println!("{}", 
                match set.range(..=k).next_back() {
                    Some(k) => k,
                    None => &-1
                });
            },
            Query::Previous {k} => {
                println!("{}", 
                match set.range(k..).next() {
                    Some(k) => k,
                    None => &-1
                });
            }
        }
    }
    Ok(())
}