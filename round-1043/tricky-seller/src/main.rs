use std::io;

fn main() {
    let mut all_pow3: Vec<u128> = Vec::new();
    let mut pow: u128 = 1;

    for _ in 0..80 {
        all_pow3.push(pow);
        pow *= 3;
    }

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t must be a u16");
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let mut result = 0;
        let mut watter_mallow: u128 = input.trim().parse().expect("t must be a u32");
        for i in (0..80).rev() {
			if watter_mallow == 0 {
				break;
			}
            if all_pow3[i] == watter_mallow {
                let val = &all_pow3[i];
                watter_mallow = 0;
                result += val * 3;
                result += i as u128 * (val / 3);
                break;
            }

            if all_pow3[i] < watter_mallow && i + 1 != 80 && all_pow3[i + 1] > watter_mallow {
                let val = &all_pow3[i];
                let del = watter_mallow / val;
                watter_mallow = watter_mallow - val * del;
                for _ in 0..del {
                    result += val * 3;
                    result += i as u128 * (val / 3);
                }
            }
        }

        if watter_mallow > 0 {
            for _ in 0..watter_mallow {
                result += 1 * 3;
            }
        }

        println!("{result}")
    }
}
