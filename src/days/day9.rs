use std::fs::read_to_string;

pub fn day9() {
    let filename = "data/day_9.txt";

    let ans: i32 = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut nums: Vec<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
            let mut diffs: Vec<i32> = Vec::new();

            while !nums.iter().all(|n| *n == 0) {
                diffs.push(*nums.first().unwrap());
                nums = nums.windows(2).map(|w| w[1] - w[0]).collect();
            }

            diffs
                .iter()
                .rev()
                .copied()
                .reduce(|acc, v| v - acc)
                .unwrap()
        })
        .sum();

    println!("{:?}", ans);
}

pub fn _day9() {
    let filename = "data/day_9.txt";

    let ans: i32 = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut nums: Vec<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
            let mut diffs: Vec<i32> = Vec::new();

            while !nums.iter().all(|n| *n == 0) {
                diffs.push(*nums.last().unwrap());
                nums = nums.windows(2).map(|w| w[1] - w[0]).collect();
            }

            diffs.iter().fold(0, |acc, v| acc + *v)
        })
        .sum();

    println!("{:?}", ans);
}
