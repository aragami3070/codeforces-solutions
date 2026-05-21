use std::io::{self, BufRead, Error, Write};
type Result<T> = std::result::Result<T, Error>;
static MUST_BE_NUMBER: &str = "Должно быть число";

fn solve<R: BufRead, W: Write>(mut reader: R, writer: &mut W) -> Result<()> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line.clear();
    reader.read_line(&mut line)?;

    let (count, ones, twos) =
        line.split_whitespace()
            .fold((0u16, 0u16, 0u16), |(count, ones, twos), num| {
                let parsed_num = num.parse::<u8>().expect(MUST_BE_NUMBER);
                match parsed_num {
                    0 => (count + 1, ones, twos),
                    1 => (count, ones + 1, twos),
                    2 => (count, ones, twos + 1),
                    _ => panic!("Должны быть числа от 0 до 2"),
                }
            });

    let maxs = std::cmp::max(ones, twos);
    let mins = std::cmp::min(ones, twos);

    writeln!(writer, "{}", count + mins + (maxs - mins) / 3)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let mut stdout = io::stdout().lock();

    for _ in 0..line.trim().parse::<usize>().expect(MUST_BE_NUMBER) {
        solve(stdin.lock(), &mut stdout)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        io::Cursor,
        time::{Duration, Instant},
    };

    use super::*;
    #[test]
    fn test_codefoces() -> Result<()> {
        let input = ["4\n0 0 0 0\n", "3\n1 2 0\n", "5\n1 2 1 2 1\n"];
        let results = ["4\n", "2\n", "2\n"];

        for (inp, res) in input.iter().zip(results.iter()) {
            let mut cur_result = Cursor::new(Vec::new());
            solve(Cursor::new(inp.as_bytes()), &mut cur_result)?;
            assert_eq!(cur_result.into_inner(), res.as_bytes());
        }

        Ok(())
    }

    #[test]
    fn my_tests() -> Result<()> {
        let input = [
            "4\n2 2 2 2\n",
            "12\n1 1 1 0 2 0 1 2 2 2 1 2\n",
            "12\n1 1 1 0 2 0 1 2 2 2 1 2 1 1 1\n",
        ];
        let results = ["1\n", "7\n", "8\n"];

        for (inp, res) in input.iter().zip(results.iter()) {
            let mut cur_result = Cursor::new(Vec::new());
            solve(Cursor::new(inp.as_bytes()), &mut cur_result)?;
            assert_eq!(cur_result.into_inner(), res.as_bytes());
        }

        Ok(())
    }

    fn timeout_check<F: FnOnce() -> Result<()>>(f: F, time_limit: u64) -> Result<()> {
        let start = Instant::now();

        f()?;

        let elapsed = start.elapsed();
        let time_limit = Duration::from_millis(time_limit);

        assert!(
            elapsed <= time_limit,
            "Не прошло по времени! Время: {:?}, лимит: {:?}",
            elapsed,
            time_limit
        );

        Ok(())
    }

    #[test]
    fn stress_test() -> Result<()> {
        let input: Vec<String> = (1..1_000_000)
            .map(|item| format!("\n{}\n", item % 3))
            .collect();

        timeout_check(
            || -> Result<()> {
                for inp in input.iter() {
                    let mut cur_result = Cursor::new(Vec::new());
                    let cur_inp = Cursor::new(inp.as_bytes());
                    solve(cur_inp, &mut cur_result)?;
                }
                Ok(())
            },
            68,
        )?;

        Ok(())
    }
}
