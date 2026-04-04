use std::io::{self, BufRead};

struct Primitive {
    counted: Vec<u128>,
}
impl Primitive {
    pub fn find_to_n(&mut self, num: usize) -> &[u128] {
        if self.counted.len() >= num {
            return &self.counted[0..num];
        }
        self.setup_first_2();
        let mut next_primitive = *self
            .counted
            .last()
            .expect("Минимум 2 элемента вычислено всегда");
        loop {
            if self.counted.len() >= num {
                break;
            }
            next_primitive += 1;
            if !next_primitive.is_multiple_of(2) && self.is_primitie(next_primitive) {
                self.counted.push(next_primitive);
            }
        }

        &self.counted[0..num]
    }

    fn is_primitie(&self, num: u128) -> bool {
        if num < 2 {
            return false;
        }
        for prime in self.counted.iter() {
            if prime * prime > num {
                break;
            }
            if num.is_multiple_of(*prime) {
                return false;
            }
        }
        true
    }

    fn setup_first_2(&mut self) {
        if self.counted.is_empty() {
            self.counted.push(2);
            self.counted.push(3);
        } else if self.counted.len() == 1 {
            self.counted.push(3);
        }
    }
}

fn solve<R: BufRead>(mut reader: R, primes: &mut Primitive) -> Vec<u128> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    primes
        .find_to_n(n + 1)
        .windows(2)
        .map(|windows| windows[0] * windows[1])
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut primes = Primitive {
        counted: Vec::new(),
    };

    for _ in 0..line.trim().parse::<usize>().unwrap() {
        let result = solve(stdin.lock(), &mut primes);
        for i in result {
            print!("{i} ");
        }
        println!("");
    }
}

// #[cfg(test)]
// mod tests {
//     use std::io::Cursor;
//
//     use super::*;
//
//     #[test]
//     fn test_example() {
//         let input = ["1\n", "3\n", "5\n"];
//         let results = vec!["2\n", "6\n", "10\n"];
//
//     // let mut primes = Primitive { counted: Vec::new() };
//     //
//     // for _ in 0..line.trim().parse::<usize>().unwrap() {
//     //     let result = solve(stdin.lock(), &mut primes);
//     //     for i in result {
//     //         print!("{i} ");
//     //     }
//     //     println!("");
//     // }
//         // for (inp, res) in input.iter().zip(results) {
//         //     assert_eq!(solve(Cursor::new(inp)), res.trim().parse::<i32>().unwrap());
//         // }
//     }
// }
