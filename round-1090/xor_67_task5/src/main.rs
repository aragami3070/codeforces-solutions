use std::io::{self, BufRead};

fn solve<R: BufRead>(mut reader: R) -> u32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();
    let lst: Vec<_> = line
        .split_whitespace()
        .map(|elem| elem.parse::<u32>().expect("Ожидались числа"))
        .collect();
    let mut max_xor = 0;
    for x in lst.iter() {
        for y in lst.iter() {
            let cur_xor = x ^ y;
            if max_xor < cur_xor {
                max_xor = cur_xor
            }
        }
    }

    max_xor
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    for _ in 0..line.trim().parse::<usize>().expect("Должно быть число") {
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
            "2\n67  67\n",
            "3\n1 2 3\n",
            "10\n67 667 167 867 267 467 367 567 767 967",
        ];
        let results = vec!["0\n", "3\n", "1012\n"];

        for (inp, res) in input.iter().zip(results) {
            assert_eq!(solve(Cursor::new(inp)), res.trim().parse::<u32>().unwrap());
        }
    }
}
