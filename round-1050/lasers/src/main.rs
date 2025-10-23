use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let t: u16 = input.trim().parse().expect("t");
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("");

        let mut inp = input.split_whitespace();

        let n: u32 = inp.next().expect("").trim().parse().expect("");
        let m: u32 = inp.next().expect("").trim().parse().expect("");
        let _x: u32 = inp.next().expect("").trim().parse().expect("");
        let _y: u32 = inp.next().expect("").trim().parse().expect("");

        let mut sum = 0;
        input.clear();
        io::stdin().read_line(&mut input).expect("");

        for _ in 0..n {
            sum += 1;
        }

        input.clear();
        io::stdin().read_line(&mut input).expect("");
        for _ in 0..m {
            sum += 1;
        }

        println!("{sum}")
    }
}
