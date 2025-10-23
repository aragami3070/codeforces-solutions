use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed input");
    let t: u16 = input.trim().parse().expect("Failed");

    for _ in 0..t {
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed input");
        let _n: u16 = input.trim().parse().expect("Failed");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");
		
		let mut result: u32 = 0;

		let mut sum_ones: u32 = 0;

		for ch in input.chars() {
			if ch == '1' {
				sum_ones += 1;
			}
		}

		for ch in input.chars() {
			if ch == '1' {
				result += sum_ones - 1;
			}
			else if ch == '0' {
			    result += sum_ones + 1;
			}
		}

		println!("{result}")
    }
}
