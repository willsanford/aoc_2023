use std::cmp::min;
use std::fs::read_to_string;

fn match_first(v: &[char]) -> Option<u32> {
    match v {
        [('0'..='9'), ..] => Some(v[0].to_digit(10).unwrap()),
        ['o', 'n', 'e', ..] => Some(1u32),
        ['t', 'w', 'o', ..] => Some(2u32),
        ['t', 'h', 'r', 'e', 'e', ..] => Some(3u32),
        ['f', 'o', 'u', 'r', ..] => Some(4u32),
        ['f', 'i', 'v', 'e', ..] => Some(5u32),
        ['s', 'i', 'x', ..] => Some(6u32),
        ['s', 'e', 'v', 'e', 'n'] => Some(7u32),
        ['e', 'i', 'g', 'h', 't'] => Some(8u32),
        ['n', 'i', 'n', 'e', ..] => Some(9u32),
        _ => None,
    }
}

fn match_last(v: &[char]) -> Option<u32> {
    match v {
        [.., ('0'..='9')] => Some(v[v.len() - 1].to_digit(10).unwrap()),
        [.., 'o', 'n', 'e'] => Some(1u32),
        [.., 't', 'w', 'o'] => Some(2u32),
        [.., 't', 'h', 'r', 'e', 'e'] => Some(3u32),
        [.., 'f', 'o', 'u', 'r'] => Some(4u32),
        [.., 'f', 'i', 'v', 'e'] => Some(5u32),
        [.., 's', 'i', 'x'] => Some(6u32),
        ['s', 'e', 'v', 'e', 'n'] => Some(7u32),
        ['e', 'i', 'g', 'h', 't'] => Some(8u32),
        [.., 'n', 'i', 'n', 'e'] => Some(9u32),
        _ => None,
    }
}

fn main() {
    let filename = "data/day_1.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .map(|line| {
            let mut n: usize = min(5, line.len());
            if line.is_empty() {
                return 0;
            }
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            while first.is_none() || last.is_none() {
                let chars: Vec<char> = line.chars().into_iter().collect();
                if first.is_none() {
                    first = chars.windows(n).find_map(match_first);
                }
                if last.is_none() {
                    last = chars.windows(n).rev().find_map(match_last);
                }
                n -= 1;
            }

            return 10 * first.unwrap() + last.unwrap();
        })
        .sum();

    println!("{:?}", ans);
}
