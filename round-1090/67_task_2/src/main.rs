use std::io::{self, BufRead};

fn solve<R: BufRead>(mut reader: R) -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|el| el.trim().parse::<i32>().unwrap())
        .collect();

    let max = nums.iter().max().expect("Всмысле 0 чисел");


    -nums.iter().sum::<i32>() + max * 2
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    for _ in 0..line.trim().parse::<usize>().unwrap() {
        let result = solve(stdin.lock());
        println!("{}", result);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_example() {
        let input = [
            "41 41 41 41 41 41 41\n",
            "6 9 4 20 6 7 67\n",
            "1 2 3 4 5 6 7\n",
            "6 7 6 7 6 7 6\n",
        ];
        let results = vec!["-205\n", "15\n", "-14\n", "-31\n"];

        for (inp, res) in input.iter().zip(results) {
            assert_eq!(solve(Cursor::new(inp)), res.trim().parse::<i32>().unwrap());
        }
    }
}
