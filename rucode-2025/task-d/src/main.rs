use std::io;
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("");
    let mut inp = input.split_whitespace();
    let x: i32 = inp.next().unwrap().trim().parse().expect("x");
    let y: i32 = inp.next().unwrap().parse().expect("y");
    let r: i32 = inp.next().unwrap().parse().expect("r");
    let mut result = 1;
    let mut flag_black = false;

    if x > 0 {
        if x - r == 0 {
            result += 1;
            flag_black = true;
        }
        if r - x > 0 {
            result += 2;
            flag_black = true;
        }
    } else if x < 0 {
        if x + r == 0 {
            result += 1;
            flag_black = true;
        }
        if r + x > 0 {
            result += 2;
            flag_black = true;
        }
    } else {
        result += 2;
        flag_black = true;
    }

    if y > 0 {
        if y - r == 0 && !flag_black {
            result += 1;
        }
        if r - y > 0 {
            result += 1;
            if !flag_black {
                result += 1
            }
        }
    } else if y < 0 {
        if y + r == 0 && !flag_black {
            result += 1;
        }
        if r + y > 0 {
            result += 1;
            if !flag_black {
                result += 1
            }
        }
    } else {
        if !flag_black {
            result += 1
        }
        result += 1
    }
    if r * r >= x * x + y * y {
        result += 1
    }

    println!("{result}")
}
