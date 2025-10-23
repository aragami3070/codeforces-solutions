use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("t should be a u16");

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).expect("Failed");

        let input_pair = input.split_once(' ').expect("Failed");

        let a: u128 = input_pair.0.trim().parse().expect("failed");
        let b: u128 = input_pair.1.trim().parse().expect("failed");

        let sum;

        if a % 2 == 0 && b % 2 != 0 {
            println!("-1");
            continue;
        } else if a % 2 != 0 && b % 2 == 0 {
            if b % 4 == 0 {
                sum = a * (b / 2) + 2;
            } else {
                println!("-1");
                continue;
            }
        } else if a % 2 != 0 && b % 2 != 0 {
            sum = a * b + 1;
        } else {
			sum = a * (b / 2) + 2;
        }

        if sum % 2 == 0 {
            println!("{sum}")
        } else {
            println!("-1")
        }
    }
}
