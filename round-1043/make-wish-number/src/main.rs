use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t must be a u64");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");
        let n: i128 = input.trim().parse().expect("n must be a u64");

        let mut tens: i128 = 10;
        let mut result: Vec<i128> = Vec::new();
        while (n / tens) > 0 {
            if n % (tens + 1) == 0 {
                result.push(n / (tens + 1));
            }
            tens *= 10;
        }

        result.sort();
        println!("{}", result.len());
        for nums in &result {
            print!("{nums} ")
        }

        if result.len() > 0 {
            println!("")
        }
    }
}
