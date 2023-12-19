use cached::proc_macro::cached;
use std::fs::read_to_string;

#[cached]
fn solve(positions: String, groups: Vec<u32>) -> u64 {
    // Break conditions
    if groups.len() == 0 {
        if positions.find('#').is_some() {
            return 0;
        } else {
            return 1;
        }
    }
    if positions.len() == 0 {
        if groups.len() > 0 {
            return 0;
        } else {
            return 1;
        }
    }

    let mut ans = 0;
    let f: char = positions.chars().nth(0).unwrap();
    if f == '.' || f == '?' {
        ans += solve(positions[1..].to_string(), groups.clone());
    }
    if f == '#' || f == '?' {
        if groups[0] <= positions.len() as u32
            && positions[0..groups[0] as usize].find('.').is_none()
            && (groups[0] == positions.len() as u32
                || positions.chars().nth(groups[0] as usize).unwrap() != '#')
        {
            let next_start = groups[0] as usize + 1;
            if next_start < positions.len() {
                ans += solve(positions[next_start..].to_string(), groups[1..].to_vec());
            } else {
                ans += solve("".to_string(), groups[1..].to_vec());
            }
        }
    }
    return ans;
}

pub fn day12() {
    let filename = "data/day_12.txt";

    let ans: u64 = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let data = line.split(' ').collect::<Vec<&str>>();
            let mut positions = data[0].to_string();
            let mut groups = data[1]
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            // Part 2
            groups = groups
                .iter()
                .cycle()
                .take(groups.len() * 5)
                .map(|x| *x)
                .collect();

            let mut extra: Vec<String> = Vec::new();
            for _ in 0..5 {
                extra.push(positions.clone());
            }
            positions = extra.join("?");

            solve(positions, groups)
        })
        .sum();

    println!("{:?}", ans);
}
