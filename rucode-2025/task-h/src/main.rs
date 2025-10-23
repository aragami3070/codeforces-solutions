use std::io::{self, Write};
fn main() {
    let mut input = String::new();

    let mut left: u32 = 0;
    let mut right: u32 = 100_000_001;
    while right - left > 1 {
        let middle = left + (right - left) / 2;

        io::stdout().flush().unwrap();
        println!("? {middle}");
        input.clear();
        io::stdin().read_line(&mut input).expect("");
        let num: i32 = input.trim().parse().expect("");
        if num > 0 {
            left = middle
        } else {
            right = middle
        }
    }
    println!("! {left}")
}
