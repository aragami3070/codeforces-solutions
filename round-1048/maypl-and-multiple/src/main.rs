use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u8 = input.trim().parse().expect("t");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let inp_pair = input.split_once(' ').expect("split");
        let a: u32 = inp_pair.0.trim().parse().expect("a");

        let b: u32 = inp_pair.1.trim().parse().expect("b");

        if a == b {
            println!("0")
        } else if a % b == 0 || b % a == 0 {
            println!("1")
        } else {
            println!("2")
        }
    }
}
