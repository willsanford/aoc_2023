use std::collections::HashSet;
use std::fs::read_to_string;

fn check_state(positions: String, groups: Vec<u32>) -> bool {
    positions.chars().filter(|c| c == &'#').count() == groups.iter().sum::<u32>() as usize
}

fn get_possible_positions(positions: String, groups: Vec<u32>) -> Vec<usize> {
    // Check every question mark and see if it will break the groups
    let mut current_groups: HashSet<u32> = groups.clone().iter().collect();
    println!(
        "{:?}",
        positions
            .split('.')
            .filter(|x| x.len() > 0)
            .collect::<Vec<&str>>()
    );
    positions
        .chars()
        .enumerate()
        .filter(|(i, c)| {
            println!("Checking {} {}", i, c);
            // Check if we can put a # here
            true
        })
        .map(|(i, c)| i)
        .collect::<Vec<usize>>()
}

fn get_possible(positions: String, groups: Vec<u32>) -> u32 {
    let mut total: u32 = 0;
    let mut to_check: Vec<String> = vec![positions.clone()];
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(positions.clone());

    while let Some(positions) = to_check.pop() {
        // Get the first element
        if check_state(positions.clone(), groups.clone()) {
            total += 1;
            continue;
        }

        // Find all the possible places where a
        let possible_positions = get_possible_positions(positions.clone(), groups.clone());

        for position in possible_positions {
            let mut new_positions = positions.clone();
            new_positions.replace_range(position..position + 1, "#");
            if !seen.contains(&new_positions) {
                to_check.push(new_positions.clone());
                seen.insert(new_positions.clone());
            }
        }

        println!("Checking {}", positions);
    }

    total
}

pub fn day12() {
    let filename = "data/day_12_ex.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let data = line.split(' ').collect::<Vec<&str>>();
            let positions = data[0].to_string();
            let groups = data[1]
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            get_possible(positions, groups)
        })
        .sum();

    println!("{:?}", ans);
}
