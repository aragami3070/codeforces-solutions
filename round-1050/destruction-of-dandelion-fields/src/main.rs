use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let t: u16 = input.trim().parse().expect("t");
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("");

        let _n: u32 = input.trim().parse().expect("t");

        input.clear();
        io::stdin().read_line(&mut input).expect("");

        let mut chet: Vec<u64> = Vec::new();
        let mut sum: u64 = 0;
        let mut nechet: Vec<u64> = Vec::new();

        for inp_str in input.split_whitespace() {
            let num: u64 = inp_str.trim().parse().expect("");

            if num % 2 == 0 {
                sum += num;
                chet.push(num);
            } else {
                nechet.push(num);
            }
        }
        nechet.sort();

        let len = nechet.len();
        if len == 0 {
            println!("0");
            continue;
        } else if len == 1 {
            sum += nechet[0];
        } else {
            for num in 0..len / 2 {
                sum += nechet[len - 1 - num];
                if len % 2 != 0 && num + 1 == len / 2 {
                    sum += nechet[num + 1];
                }
            }
        }

        println!("{sum}")
    }
}
