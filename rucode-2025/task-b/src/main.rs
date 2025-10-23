use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("");

    if let Some(_) = input.find(':') {
        let mut res = 0;
        for (index, nums) in input.split(':').enumerate() {
            let n: u32 = nums.trim().parse().expect("");
            match index {
                0 => res += n * 3600,
                1 => res += n * 60,
                _ => res += n,
            }
        }
        println!("{res}")
    } else {
        let mut hours = 0;
        let mut minuts = 0;
        let seconds;

        let mut inp: u32 = input.trim().parse().expect("");

        if inp / 3600 > 0 {
            hours = inp / 3600;
            inp -= hours * 3600;
        }
        if inp / 60 > 0 {
            minuts = inp / 60;
            inp -= minuts * 60;
        }
        seconds = inp;

        let mut format_str = String::new();
        if hours < 10 {
            format_str.push_str("0");
        }
        format_str.push_str(&hours.to_string());
        format_str.push_str(":");
        if minuts < 10 {
            format_str.push_str("0");
        }
        format_str.push_str(&minuts.to_string());
        format_str.push_str(":");
        if seconds < 10 {
            format_str.push_str("0");
        }
        format_str.push_str(&seconds.to_string());

        println!("{format_str}")
    }
}
