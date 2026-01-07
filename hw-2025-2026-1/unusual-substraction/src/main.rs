use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let (num_str_1, num_str_2) = input.trim().split_once(' ').unwrap();
    let mut num1: u64 = num_str_1.trim().parse()?;
    let mut num2: u64 = num_str_2.trim().parse()?;

    loop {
        if num1 == 0 || num2 == 0 {
            break;
        }

        if num1 >= 2 * num2 {
            num1 -= 2 * num2 * (num1 / (2 * num2))
        } else if num2 >= 2 * num1 {
            num2 -= 2 * num1 * (num2 / (2 * num1))
        }
        else {
            break;
        }
    }

    println!("{num1} {num2}");
    Ok(())
}
