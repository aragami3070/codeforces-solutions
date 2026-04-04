use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let t: u16 = input.trim().parse()?;

    for _ in 0..t {
        io::stdin().read_line(&mut input)?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let dates: Vec<u32> = input
            .split_whitespace()
            .map(|num_str| num_str.trim().parse())
            .collect::<Result<_, _>>()?;
        let mut year: u32 = 0;

        for &date_i in dates.iter() {
            if year >= date_i {
                year = date_i + date_i * (year / date_i)
            } else {
                year = date_i
            }
        }
        println!("{year}")
    }

    Ok(())
}
