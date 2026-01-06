use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let number: u128 = input.trim().parse()?;

    let mut i = 2;

    while i * i <= number {
        if number.is_multiple_of(i) {
            println!("0");
            return Ok(());
        }

        i += 1;
    }

    println!("1");
    Ok(())
}
