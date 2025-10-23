use std::{io, usize};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let t: u16 = input.trim().parse().expect("Failed");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");
        let n: u32 = input.trim().parse().expect("Failed");

        let mut nums: Vec<(u64, usize, Vec<usize>, u64)> =
            vec![(0, 0, Vec::new(), 0); (n + 1) as usize];
        let mut arr: Vec<(u64, bool)> = Vec::new();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed");

        let mut before: u64 = 0;
        let mut before_again: u64 = 0;

        for (index, str_inp) in input.split_whitespace().enumerate() {
            let num: u64 = str_inp.trim().parse().expect("Failed");
            if num != before && before_again != 0 {
                if nums[num as usize].2.len() > 0 && nums[num as usize].2.len() % 2 != 0 {
                    nums[num as usize].2.push(index - 1);
                } else if nums[before as usize].0 % before != 0 {
                    nums[before as usize].2.push(index);
                }
                before_again = 0;
            }

            nums[num as usize].0 += 1;
            nums[num as usize].1 = num as usize;
            arr.push((num, true));
            before = num;
            before_again += 1;
            nums[num as usize].3 =
                nums[num as usize].0 - (nums[num as usize].0 % nums[num as usize].1 as u64);
        }

        nums.sort_by(|a, b| (a.3).cmp(&(b.3)));

        let mut result: u64 = 0;

        for i in (0..n + 1).rev() {
            let len = nums[i as usize].2.len();
            let val = nums[i as usize].1;
            let max = &nums[i as usize].3;
            if len > 1 && *max > 0 {
                for j in (0..len).step_by(2) {
                    if j + 1 == len {
                        nums[i as usize].3 -= val as u64;
                        break;
                    } else {
                        let right = nums[i as usize].2[j + 1];
                        let left = nums[i as usize].2[j];
                        let can_use = arr[left].1 || arr[right].1;

                        if right - left < val && can_use {
                            for k in left..right + 1 {
                                arr[k].1 = false;
                                let index =
                                    nums.iter().position(|r| r.1 == arr[k].0 as usize).unwrap();
                                nums[index].3 -= nums[index].1 as u64;
                            }
                        } else {
                            nums[i as usize].3 -= val as u64;
                        }
                    }
                }
                nums[i as usize].2.clear();
            }
            result += nums[i as usize].3
        }

        nums.clear();
        println!("{result}")
    }
}
