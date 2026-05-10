use std::io::{self, BufRead};

fn solve<R: BufRead>(mut reader: R) -> &'static str {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let (x, y): (u32, u32) = line
        .trim()
        .split_once(' ')
        .map(|(x_str, y_str)| {
            (
                x_str.parse::<u32>().expect("Ожидалось число"),
                y_str.parse::<u32>().expect("Ожидалось число"),
            )
        })
        .expect("Ожидалась пара чисел");
    if x % 2 != 0 && y % 2 != 0 {
        "NO"
    } else {
        "YES"
    }
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
    fn test_from_codeforces() {
        let input = ["1 1\n", "1 2\n", "4 6\n", "5 9\n", "7 2\n", "10 10\n"];
        let results = vec!["NO\n", "YES\n", "YES\n", "NO\n", "YES\n", "YES\n"];

        for (inp, res) in input.iter().zip(results) {
            assert_eq!(solve(Cursor::new(inp)), res.trim());
        }
    }
}
