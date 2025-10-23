use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let t: u16 = input.trim().parse().expect("t");
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("");

        let pair_n_m = input.split_once(' ').expect("");

        let n: u32 = pair_n_m.0.trim().parse().expect("");
        let m: u32 = pair_n_m.1.trim().parse().expect("");

        let mut sum = 0;
        let mut minut = 0;
        let mut side = 0;
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).expect("");
            let pair = input.split_once(' ').expect("");
            let a: u32 = pair.0.trim().parse().expect("");
            let b: u8 = pair.1.trim().parse().expect("");

            let time_dif = a - minut;
            if b == side {
				if time_dif % 2 == 0 {
					sum += time_dif;
				}
				else {
				    sum += time_dif - 1;
				}
            } else {
				if time_dif % 2 == 1 {
					sum += time_dif;
				}
				else {
				    sum += time_dif - 1;
				}
            }

            minut = a;
            side = b;
            if i == n - 1 {
                sum += m - minut;
            }
        }

        println!("{sum}")
    }
}
