use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("Failed");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let _n: u8 = input.trim().parse().expect("Failed");
        let mut max_sum: u16 = 0;
		let mut zeros: u16 = 0;

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        for str_inp in input.split_whitespace() {
            let num: u8 = str_inp.trim().parse().expect("Failed");
			if num == 0 {
				zeros += 1;
			}
            max_sum += num as u16;
        }

        if zeros >= 1 {
            max_sum += zeros;
        }

        println!("{max_sum}")
    }
}
