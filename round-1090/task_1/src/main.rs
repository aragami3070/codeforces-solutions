use std::io::{self, BufRead};

fn solve<R: BufRead>(mut reader: R) -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    67
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
        let input = ["1\n", "3\n", "5\n"];
        let results = vec!["67\n", "67\n", "67\n"];

        for (inp, res) in input.iter().zip(results) {
            assert_eq!(solve(Cursor::new(inp)), res.trim().parse::<i32>().unwrap());
        }
    }
}
