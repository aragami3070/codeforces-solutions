use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let t: u16 = input.trim().parse().expect("Failed");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let n: u32 = input.trim().parse().expect("Failed");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let p_vec: Vec<u32> = input
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Failed"))
            .collect();

        for i in p_vec {
            print!("{} ", n + 1 - i);
        }
        println!()
    }
}
