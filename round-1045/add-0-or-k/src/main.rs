use std::io;

fn gcd(n: &u32, m: &u32) -> u32 {
    let mut f = n.clone();
    let mut s = m.clone();

    while s != 0 {
        if s < f {
            let t = s;
            s = f;
            f = t;
        }
        s = s % f;
    }
    f
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("Failed");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let temp = input.split_once(" ").expect("Failed");
        let n: u32 = temp.0.trim().parse().expect("Failed");
        let k: u32 = temp.0.trim().parse().expect("Failed");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let arr_a: Vec<u32> = input
            .split_whitespace()
            .map(|num| num.parse::<u32>().expect("Failed"))
            .collect();


    }
}
