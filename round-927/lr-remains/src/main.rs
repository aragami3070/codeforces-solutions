use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed input t");

    let t: u16 = input.trim().parse().expect("t will be u16");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let pair_value = input.split_once(' ').expect("Failed");
        let n: u32 = pair_value.0.trim().parse().expect("n will be u32");
        let m: u32 = pair_value.1.trim().parse().expect("n will be u16");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let arr: Vec<u16> = input
            .split_whitespace()
            .map(|num| num.parse::<u16>().expect("a will be u16"))
            .collect();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let lr_str = &input.trim();

        let mut left: isize = 0;
        let mut right: isize = n as isize - 1;
        let mut end = 0;

        for (index, ch) in lr_str.chars().enumerate() {
            if index + 1 == n as usize {
                end = if ch == 'L' { left } else { right };
                break;
            }
            if ch == 'L' {
                left += 1;
            } else {
                right -= 1;
            }
        }

        let mut mult: u32 = 1;
        let mut result: Vec<u32> = Vec::new();
        for (index, ch) in lr_str.chars().rev().enumerate() {
            if index == 0 {
                left = end as isize;
                right = end;
                let new_remaind = arr[end as usize] as u32 % m;
                mult *= new_remaind;
                result.push(mult % m);
                continue;
            }
            if ch == 'L' {
                left -= 1;
                let new_remaind = arr[left as usize] as u32 % m;
                mult *= new_remaind;
                result.push(mult % m);
            } else {
                right += 1;
                let new_remaind = arr[right as usize] as u32 % m;
                mult *= new_remaind;
                result.push(mult % m);
            }
        }

        for index in (0..result.len()).rev() {
            print!("{} ", result[index])
        }
        println!()
    }
}
