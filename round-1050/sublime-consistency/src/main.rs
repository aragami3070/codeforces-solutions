use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let t: u8 = input.trim().parse().expect("t");
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("");

        let inp_s = input.split_once(' ').expect("");
		let n = inp_s.1.trim().parse::<u8>().expect("");
		let x = inp_s.0.trim().parse::<u8>().expect("");

		if n % 2 == 0 {
			println!("0")
		}
		else {
		    println!("{x}")
		}
    }
}
