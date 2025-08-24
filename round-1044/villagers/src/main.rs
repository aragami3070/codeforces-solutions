

use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t should be a u16");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");
        let _n: u32 = input.trim().parse().expect("n shoold be a u32");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let mut people: Vec<u64> = input
            .split_whitespace()
            .map(|num| num.trim().parse::<u64>().expect("Failed parse"))
            .collect();
        people.sort();

        let mut sum = 0;
        for i in (0..people.len()).rev().step_by(2) {
            sum += people[i];
        }
        println!("{sum}")
    }
}
