use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t should be a u16");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");
        let _n: u8 = input.trim().parse().expect("n shoold be a u8");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let mut flag = false;
		let mut index = (0, 0);
        for first_gear in input.split_whitespace() {
            for last_gear in input.split_whitespace() {
				let first = first_gear.trim().parse::<f64>().expect("Failed parse");
				let last = last_gear.trim().parse::<f64>().expect("Failed parse");
                if index.0 != index.1 && first / last == 1.0 {
                    flag = true;
                    break;
                }
				index.1 += 1;
            }
			if flag {
				break;
			}
			index.0 += 1;
			index.1 = 0;
        }

        if flag {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
