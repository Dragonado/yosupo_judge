use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum Query {
    Set { k: i64, v: i64 },
    Get { k: i64 }
}

fn read_one_i64() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().unwrap();

    a
}

fn read_query() -> Query {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let t: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();

    match t {
        0 => {
            let v: i64 = iter.next().unwrap().parse().unwrap();
            Query::Set {k, v }
        },
        1 => Query::Get {k},
        _ => todo!()
    }
}

fn main() {
    let mut t = read_one_i64();
    let mut hs = HashMap::new();

    while { let tmp = t; t -= 1; tmp } > 0 {
        let q = read_query();
        match q {
            Query::Get { k } => println!("{}", match hs.get(&k) {
                Some(k) => k,
                None => &0
            }),
            Query::Set {k, v} => {hs.insert(k, v); ()}
        }
    }
}