use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let t: u16 = input.trim().parse()?;

    for _ in 0..t {
        io::stdin().read_line(&mut input)?;
        input.clear();
        io::stdin().read_line(&mut input)?;
        let nums: Vec<u8> = input
            .split_whitespace()
            .map(|n| n.trim().parse())
            .collect::<Result<_, _>>()?;

        let mut all_ones = true;
        let mut all_zeros = true;
        let mut have_zeros = false;

        for (index, &i) in nums.iter().enumerate() {
            if all_ones && i == 0 {
                all_ones = false
            }
            if all_zeros && i == 1 {
                all_zeros = false
            }
            if !have_zeros && index != 0 && index != nums.len() - 1 && i == 0 {
                have_zeros = true
            }
        }
        let check = nums[0] + nums[nums.len() - 1];

        if !all_zeros && (all_ones || check == 1 || (check == 2 && have_zeros)) {
            println!("Alice")
        } else {
            println!("Bob")
        }
    }

    Ok(())
}
