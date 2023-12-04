use std::collections::HashSet;
use std::fs::read_to_string;

pub fn day4() {
    let filename = "data/day_4.txt";

    let mut nums: Vec<(u32, u32)> = read_to_string(filename)
        .unwrap()
        .trim()
        .split('\n')
        .into_iter()
        .map(|line| {
            let numbers = line.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .collect::<Vec<&str>>();
            let winning_numbers = numbers[0]
                .trim()
                .split(' ')
                .map(|x| x.trim().parse::<u32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect::<HashSet<u32>>();

            let our_numbers = numbers[1]
                .trim()
                .split(' ')
                .map(|x| x.trim().parse::<u32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect::<HashSet<u32>>();
            (winning_numbers.intersection(&our_numbers).count() as u32, 1)
        })
        .collect();

    let mut ans = 0;
    for i in 0..nums.len() {
        ans += nums[i].1;
        for j in i + 1..=i + nums[i].0 as usize {
            nums[j].1 += nums[i].1;
        }
    }
    println!("{:?}", ans);
}

pub fn _day4() {
    let filename = "data/day_4.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .trim()
        .split('\n')
        .into_iter()
        .map(|line| {
            let numbers = line.split(':').collect::<Vec<&str>>()[1]
                .split('|')
                .collect::<Vec<&str>>();
            let winning_numbers = numbers[0]
                .trim()
                .split(' ')
                .map(|x| x.trim().parse::<u32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect::<HashSet<u32>>();

            let our_numbers = numbers[1]
                .trim()
                .split(' ')
                .map(|x| x.trim().parse::<u32>())
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect::<HashSet<u32>>();
            if let Some(n_) = winning_numbers
                .intersection(&our_numbers)
                .count()
                .checked_sub(1)
            {
                u32::pow(2, n_ as u32)
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", ans);
}
