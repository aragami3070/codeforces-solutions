use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let t: u16 = input.trim().parse()?;

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let (num_str_1, num_str_2) = input.trim().split_once(' ').unwrap();
        let mut num1: u64 = num_str_1.trim().parse()?;
        let mut num2: u64 = num_str_2.trim().parse()?;
        let temp = num1;

        let mut count: u64 = 0;

        loop {
            if num2 == 0 || num1 == 0 {
                break;
            }

            if num2 > num1 {
                let num_of_times = num2 / num1;
                count += num_of_times;
                num2 -= num1 * num_of_times
            } else {
                let num_of_times = num1 / num2;
                count += num_of_times;
                num1 -= num2 * num_of_times
            }
        }
        if num1 == num2 {
            println!("{count} {temp}");
            continue;
        }
        println!("{count} {}", num1.max(num2))
    }

    Ok(())
}
