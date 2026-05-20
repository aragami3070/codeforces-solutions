use std::io::{self, BufRead, Error, Write};

type Result<T> = std::result::Result<T, Error>;

static MUST_BE_NUMBER: &str = "Должно быть число";

fn solve<R: BufRead, W: Write>(mut reader: R, writer: &mut W) -> Result<()> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line.clear();
    reader.read_line(&mut line)?;
    let nums: Vec<u64> = line
        .split_whitespace()
        .map(|num| num.trim().parse().expect(MUST_BE_NUMBER))
        .collect();

    for mult_of_6 in nums.iter().filter(|&&num| num % 6 == 0) {
        write!(writer, "{mult_of_6} ")?
    }

    for mult_of_3 in nums.iter().filter(|&&num| num % 3 == 0 && num % 6 != 0) {
        write!(writer, "{mult_of_3} ")?
    }

    for odd_without_3 in nums.iter().filter(|&&num| num % 2 != 0 && num % 3 != 0) {
        write!(writer, "{odd_without_3} ")?
    }

    for even_without_6 in nums.iter().filter(|&&num| num % 2 == 0 && num % 6 != 0) {
        write!(writer, "{even_without_6} ")?
    }

    Ok(())
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
    use std::io::Cursor;

    use super::*;
    #[test]
    fn test_example() -> Result<()> {
        let input = [
            "6\n12 7 9 4 18 5\n",
            "4\n3 6 2 8\n",
            "7\n1 10 15 20 3 6 9\n",
            "5\n11 14 21 2 5\n",
            "3\n6 6 6\n",
        ];
        let results = [
            "12 18 9 7 5 4 ",
            "6 3 2 8 ",
            "6 15 3 9 1 10 20 ",
            "21 11 5 14 2 ",
            "6 6 6 ",
        ];

        for (inp, res) in input.iter().zip(results.iter()) {
            let mut cur_result = Cursor::new(Vec::new());
            solve(Cursor::new(inp.as_bytes()), &mut cur_result)?;
            assert_eq!(
                cur_result.clone().into_inner(),
                res.as_bytes(),
                "Правильный ответ: {res:?}; Текущий: {cur_result:?}"
            );
        }

        Ok(())
    }
}
