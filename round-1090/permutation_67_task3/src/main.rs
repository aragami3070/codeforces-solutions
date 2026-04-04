use std::io::{self, BufRead};

fn solve<R: BufRead>(mut reader: R) -> Vec<(i32, i32, i32)> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    (1..=n)
        .zip((n + 1..=3 * n).collect::<Vec<i32>>().chunks(2))
        .map(|(i, wind)| (i, wind[0], wind[1]))
        .collect()
}

#[allow(clippy::println_empty_string)]
fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    for _ in 0..line.trim().parse::<usize>().unwrap() {
        let result = solve(stdin.lock());
        for (i, j, k) in result {
            print!("{i} {j} {k} ");
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_example() {
        let input = ["2\n", "1\n", "3\n"];
        let results = vec![
            vec![(1, 3, 4), (2, 5, 6)],
            vec![(1, 2, 3)],
            vec![(1, 4, 5), (2, 6, 7), (3, 8, 9)],
        ];

        for (inp, res) in input.iter().zip(results) {
            let results = solve(Cursor::new(inp));
            for (ind, trio) in results.iter().enumerate() {
                assert_eq!(*trio, res[ind])
            }
        }
    }
}
