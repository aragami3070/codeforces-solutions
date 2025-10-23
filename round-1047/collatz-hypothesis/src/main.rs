use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("Failed t input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let input_pair = input.split_once(' ').expect("Failed input");

        let k: u8 = input_pair.0.trim().parse().expect("Failed k input");
        let mut x: u64 = input_pair.1.trim().parse().expect("Failed x input");

        for _ in 0..k {
            x *= 2;
        }
        println!("{x}")
    }
}
