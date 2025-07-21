use std::io;

fn read_one_i128() -> i128 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i128 = iter.next().unwrap().parse().unwrap();

    a
}

fn read_two_i128() -> (i128, i128) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: i128 = iter.next().unwrap().parse().unwrap();
    let b: i128 = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn main() {
    let mut t = read_one_i128();

    while { let tmp = t; t -= 1; tmp } > 0 {
        let (a, b) = read_two_i128();
        println!("{}", a + b);
    }
}