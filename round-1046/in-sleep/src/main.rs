use std::{cmp, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("Failed");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let mut fours = input.split_whitespace();

        let a: i16 = fours.next().expect("").parse().expect("Failed");
        let b: i16 = fours.next().expect("").parse().expect("Failed");
        let c: i16 = fours.next().expect("").parse().expect("Failed");
        let d: i16 = fours.next().expect("").parse().expect("Failed");

        let min_first = cmp::min(b, a) * 2 + 2;
        let max_first = cmp::max(b, a);
        let min_second = cmp::min(d - b, c - a) * 2 + 2;
        let max_second = cmp::max(d - b, c - a);

        if min_first < max_first || min_second < max_second {
            println!("NO");
            continue;
        }
        println!("YES")
    }
}
