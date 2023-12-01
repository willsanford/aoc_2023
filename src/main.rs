use std::cmp::min;
use std::fs::read_to_string;
fn main() {
    let filename = "data/day_1.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .map(|line| {
            let n: usize = min(1, line.len());
            if line.is_empty() {
                return 0;
            }
            let chars: Vec<char> = line.chars().into_iter().collect();
            let first: Option<u32> = chars.windows(n).find_map(|v| match v {
                [('0'..='9'), ..] => Some(v[0].to_digit(10).unwrap()),
                _ => None,
            });
            let last: Option<u32> = chars.windows(n).rev().find_map(|v| match v {
                [.., ('0'..='9')] => Some(v[n - 1].to_digit(10).unwrap()),
                _ => None,
            });
            return 10 * first.unwrap() + last.unwrap();
        })
        .sum();

    println!("{:?}", ans);
}
