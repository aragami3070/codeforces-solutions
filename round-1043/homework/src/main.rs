use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t must be a u16");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let _n: u8 = input.trim().parse().expect("n must be a u8");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let first_str = input.clone();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let _m: u8 = input.trim().parse().expect("n must be a u8");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let second_str = input.clone();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let mut vlad = String::new();
        let mut dima = String::new();

        for (index, i) in input.chars().enumerate() {
            if i == 'V' {
                vlad.push_str(&second_str[index..index + 1]);
            }
            if i == 'D' {
                dima.push_str(&second_str[index..index + 1]);
            }
        }

        vlad = vlad.chars().rev().collect();
        vlad.push_str(first_str.trim());
        vlad.push_str(dima.trim());
        println!("{vlad}")
    }
}
