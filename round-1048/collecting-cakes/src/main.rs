use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("t");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let inp_pair = input.split_once(' ').expect("split");
        let n: u128 = inp_pair.0.trim().parse().expect("n");
        let mut m: u128 = inp_pair.1.trim().parse().expect("m");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let mut a_vec: Vec<u128> = input
            .split_whitespace()
            .map(|num| num.parse::<u128>().expect("parse failed"))
            .collect();
		a_vec.sort();

        let mut result: u128 = 0;
        let mut seconds: u128 = 0;
        if m > n {
            let div = m - n + 1;
            result += a_vec[0] * div;
            seconds += div;
            m -= div;
        }

        let mut index = n - m;
        while index < n {
            seconds += 1;
            result += seconds * a_vec[index as usize];
            index += 1;
        }

        println!("{result}")
    }
}
