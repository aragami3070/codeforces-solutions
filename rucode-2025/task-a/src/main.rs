use std::io;

fn main() {
	let mut input = String::new();

	io::stdin().read_line(&mut input).expect("");
	let a: u16 = input.trim().parse().expect("a");

	input.clear();
	io::stdin().read_line(&mut input).expect("");
	let b: u16 = input.trim().parse().expect("b");

	let res = a * 10 / b % 10;
	println!("{res}")
}
