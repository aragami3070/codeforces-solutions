use std::io::{self, BufRead};

#[inline]
fn solve<R: BufRead>(mut reader: R) -> &'static str {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.clear();
    reader.read_line(&mut line).unwrap();

    let all_brace = line.trim().len();
    let left_brace = line
        .as_bytes()
        .iter()
        .filter(|&ch| (*ch as char) == '(')
        .count();

    if all_brace - left_brace == left_brace {
        "YES"
    } else {
        "NO"
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
        let input = [
            "2\n ()\n",
            "2\n)(\n",
            "3\n (((\n",
            "6\n ())(()\n",
            "4\n (()(\n",
            "5\n)()()\n",
        ];
        let results = vec!["YES", "YES", "NO", "YES", "NO", "NO"];

        for (inp, res) in input.iter().zip(results) {
            assert_eq!(solve(Cursor::new(inp)), res.trim());
        }
    }
}
