use std::io::{self, BufRead, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static ARG_MUST_BE: &str = "Должен быть еще аргумент в input (не правильный формат)";

fn solve<R: BufRead, W: Write>(mut reader: R, writer: &mut W) -> Result<()> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut args = line.split_whitespace();

    let positions_n: i64 = args.next().expect(ARG_MUST_BE).parse()?;
    let reimy_start: i64 = args.next().expect(ARG_MUST_BE).parse()?;
    let remilia_start: i64 = args.next().expect(ARG_MUST_BE).parse()?;
    let remilia_steps: i64 = args.next().expect(ARG_MUST_BE).parse()?;

    let result_sec = match positions_n {
        i64::MIN..=1 => panic!("n should be 2 or more"),
        2..=3 => 1,
        _ => {
            let direct_distance = (reimy_start - remilia_start).abs();
            std::cmp::min(positions_n - direct_distance, direct_distance) + remilia_steps
        }
    };

    Ok(writeln!(writer, "{}", result_sec)?)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    let mut stdout = io::stdout().lock();

    for _ in 0..line.trim().parse::<usize>()? {
        solve(stdin.lock(), &mut stdout)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn codeforces_test() -> Result<()> {
        let input = ["2 1 2 0\n", "4 3 2 1\n", "4 2 3 1\n", "16 8 4 2\n"];
        let results = ["1\n", "2\n", "2\n", "6\n"];

        for (inp, res) in input.iter().zip(results.iter()) {
            let mut cur_result = Cursor::new(Vec::new());
            solve(Cursor::new(inp.as_bytes()), &mut cur_result)?;
            assert_eq!(cur_result.into_inner(), res.as_bytes());
        }

        Ok(())
    }

    #[test]
    fn corner_cases_test() -> Result<()> {
        let input = [
            "2 1 2 1000\n",
            "100000000 100000000 2 1\n",
            "4 1 3 1\n",
            "3 1 3 1000\n",
        ];
        let results = ["1\n", "3\n", "3\n", "1\n"];

        for (inp, res) in input.iter().zip(results.iter()) {
            let mut cur_result = Cursor::new(Vec::new());
            solve(Cursor::new(inp.as_bytes()), &mut cur_result)?;
            assert_eq!(cur_result.into_inner(), res.as_bytes());
        }

        Ok(())
    }

    #[test]
    fn big_test() -> Result<()> {
        let input = [
            "2 1 2 2\n",
            "2 2 1 2\n",
            "3 1 2 2\n",
            "3 1 3 1\n",
            "3 2 1 2\n",
            "3 2 3 0\n",
            "3 3 1 1\n",
            "3 3 2 0\n",
            "4 1 2 2\n",
            "4 1 3 1\n",
            "4 1 4 0\n",
            "4 2 1 2\n",
            "4 2 3 0\n",
            "4 3 1 1\n",
            "4 3 2 0\n",
            "4 4 1 0\n",
            "5 1 2 2\n",
            "5 1 3 1\n",
            "5 1 4 0\n",
            "5 2 1 2\n",
            "5 2 3 0\n",
            "5 3 1 1\n",
            "5 3 2 0\n",
            "5 4 1 0\n",
            "2 1 2 3\n",
            "2 2 1 3\n",
            "3 1 2 3\n",
            "3 1 3 2\n",
            "3 2 1 3\n",
            "3 2 3 1\n",
            "3 3 1 2\n",
            "3 3 2 1\n",
            "4 1 2 3\n",
            "4 1 3 2\n",
            "4 1 4 1\n",
            "4 2 1 3\n",
            "4 2 3 1\n",
            "4 2 4 0\n",
            "4 3 1 2\n",
            "4 3 2 1\n",
            "4 4 1 1\n",
            "4 4 2 0\n",
            "5 1 2 3\n",
            "5 1 3 2\n",
            "5 1 4 1\n",
            "5 1 5 0\n",
            "5 2 1 3\n",
            "5 2 3 1\n",
            "5 2 4 0\n",
            "5 3 1 2\n",
            "5 3 2 1\n",
            "5 4 1 1\n",
            "5 4 2 0\n",
            "5 5 1 0\n",
            "6 1 2 3\n",
            "6 1 3 2\n",
        ];
        let results = [
            "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "3\n", "3\n", "1\n", "3\n",
            "1\n", "3\n", "1\n", "1\n", "3\n", "3\n", "2\n", "3\n", "1\n", "3\n", "1\n", "2\n",
            "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "1\n", "4\n", "4\n", "2\n", "4\n",
            "2\n", "2\n", "4\n", "2\n", "2\n", "2\n", "4\n", "4\n", "3\n", "1\n", "4\n", "2\n",
            "2\n", "4\n", "2\n", "3\n", "2\n", "1\n", "4\n", "4\n",
        ];
        for (inp, res) in input.iter().zip(results.iter()) {
            let mut cur_result = Cursor::new(Vec::new());
            solve(Cursor::new(inp.as_bytes()), &mut cur_result)?;
            let parsed_res = String::from_utf8(cur_result.into_inner()).unwrap();
            assert_eq!(&parsed_res, res);
        }

        Ok(())
    }
}
