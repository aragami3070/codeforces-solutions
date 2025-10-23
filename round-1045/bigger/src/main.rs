use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t should be a u16");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let n: u32 = input.trim().parse().expect("t should be a u16");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");
    }
}
