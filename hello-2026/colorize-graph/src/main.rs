use std::{collections::HashMap, error::Error, io};

pub struct Graph {
    pub nodes: HashMap<u32, Vec<u32>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let t: u16 = input.trim().parse()?;

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let n: u32 = input.trim().parse()?;

        let mut graph = Graph {
            nodes: HashMap::new(),
        };
        let mut origin = 0;

        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input)?;
            let (num_str_1, num_str_2) = input.trim().split_once(' ').unwrap();
            let mut num1: u64 = num_str_1.trim().parse()?;
            let mut num2: u64 = num_str_2.trim().parse()?;
            if graph.nodes.len() == 0 {
                origin = num1
            }
        }
    }

    Ok(())
}
