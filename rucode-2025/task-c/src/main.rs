use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("");
    let n: u16 = input.trim().parse().expect("n");

    let mut format_str = String::new();
    format_str.push_str("80");
    for _ in 0..n - 1 {
        format_str.push_str("00");
    }
    println!("{format_str}")
}
