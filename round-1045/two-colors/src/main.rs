use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed input");

    let t: u16 = input.trim().parse().expect("t should be a u16");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed input");

        let mut temp = input.split_whitespace();

        let n: u32 = temp.next().unwrap().trim().parse().expect("Failed input");
        let a: u32 = temp.next().unwrap().trim().parse().expect("Failed input");
        let b: u32 = temp.next().unwrap().trim().parse().expect("Failed input");

        if n % 2 == 0 && (a % 2 == 0 || a < b) && b % 2 == 0 {
            println!("YES")
        } else if n % 2 != 0 && (a % 2 != 0 || a < b) && b % 2 != 0 {
            println!("YES")
        } else {
            println!("NO")
        }
    }
}
