use std::io;

fn read_one_i64() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().unwrap();

    a
}

fn read_two_i64() -> (i64, i64) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i64 = iter.next().unwrap().parse().unwrap();
    let b: i64 = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn main() {
    let mut t = read_one_i64();

    while { let tmp = t; t -= 1; tmp } > 0 {
        let (a, b) = read_two_i64();
        println!("{}", a + b);
    }
}