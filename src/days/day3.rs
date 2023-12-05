use std::collections::HashSet;
use std::fs::read_to_string;

fn group_vector(vec: &Vec<(usize, &str)>) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut current_sum = 0;
    let mut current_str = String::new();
    let mut prev_num = None;

    for (num, s) in vec {
        match prev_num {
            Some(prev) if num - prev > 1 => {
                // If the difference is greater than 1, start a new group
                if current_sum == 0 {
                    current_sum = vec[0].0;
                }
                result.push((current_sum, current_str.clone()));
                current_sum = *num;
                current_str = s.to_string();
            }
            _ => {
                // Continue the current group
                current_str.push_str(s);
            }
        }
        prev_num = Some(num);
    }

    // Add the last group to the result
    if !current_str.is_empty() {
        if current_sum == 0 {
            current_sum = vec[0].0;
        }
        result.push((current_sum, current_str));
    }

    result
}

pub fn day3() {
    let filename = "data/day_3.txt";
    let bindiung = read_to_string(filename).unwrap();

    let input: Vec<&str> = bindiung.trim().split('\n').collect();
    let data: Vec<Vec<char>> = input
        .clone()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    // scan for gear characters
    let mut stars: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '*' {
                stars.insert((i, j));
            }
        }
    }

    let nums: Vec<Vec<(usize, &str)>> = input
        .iter()
        .map(|line| {
            line.rmatch_indices(|c| c >= '0' && c <= '9')
                .rev()
                .collect()
        })
        .collect();

    let mut sum = 0;
    let nums: Vec<Vec<(usize, usize, u32)>> = nums
        .iter()
        .map(|line| {
            group_vector(&line)
                .iter()
                .map(|(start, n)| {
                    let n_ = n.parse::<u32>().unwrap();
                    (*start, n.len(), n_)
                })
                .collect()
        })
        .collect();
    for (i, j) in stars {
        // Check all the squares around the star
        let mut prod = 1;
        let mut count = 0;

        if let Some(d) = i.checked_sub(1) {
            for num in nums[d].iter() {
                let mut lower = 0;
                if let Some(j_) = num.0.checked_sub(1) {
                    lower = j_;
                }
                if j >= lower && j <= num.0 + num.1 {
                    println!("num: {:?}", num);
                    prod *= num.2;
                    count += 1;
                }
            }
        }

        for num in nums[i].iter() {
            let mut lower = 0;
            if let Some(j_) = num.0.checked_sub(1) {
                lower = j_;
            }
            if j >= lower && j <= num.0 + num.1 {
                println!("num: {:?}", num);
                prod *= num.2;
                count += 1;
            }
        }
        if i + 1 < nums.len() {
            for num in nums[i + 1].iter() {
                let mut lower = 0;
                if let Some(j_) = num.0.checked_sub(1) {
                    lower = j_;
                }
                if j >= lower && j <= num.0 + num.1 {
                    println!("num: {:?}", num);
                    prod *= num.2;
                    count += 1;
                }
            }
        }
        println!("i: {}, j: {}, prod: {}, count: {}", i, j, prod, count);
        if count == 2 {
            sum += prod;
        }
    }
    println!("Part 2: {}", sum);
}

pub fn _day3() {
    let filename = "data/day_3.txt";
    let bindiung = read_to_string(filename).unwrap();

    let input: Vec<&str> = bindiung.trim().split('\n').collect();
    let data: Vec<Vec<char>> = input
        .clone()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    // scan for gear characters
    let mut possible_gears: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            match data[i][j] {
                '.' => {}
                '0'..='9' => {}
                _ => {
                    for i_ in 0..=1 {
                        for j_ in 0..=1 {
                            possible_gears.insert((
                                i.checked_add(i_).unwrap_or(i),
                                j.checked_add(j_).unwrap_or(j),
                            ));
                            possible_gears.insert((
                                i.checked_sub(i_).unwrap_or(i),
                                j.checked_add(j_).unwrap_or(j),
                            ));
                            possible_gears.insert((
                                i.checked_add(i_).unwrap_or(i),
                                j.checked_sub(j_).unwrap_or(j),
                            ));
                            possible_gears.insert((
                                i.checked_sub(i_).unwrap_or(i),
                                j.checked_sub(j_).unwrap_or(j),
                            ));
                        }
                    }
                }
            }
        }
    }

    let nums: Vec<Vec<(usize, &str)>> = input
        .iter()
        .map(|line| {
            line.rmatch_indices(|c| c >= '0' && c <= '9')
                .rev()
                .collect()
        })
        .collect();

    let mut sum = 0;
    for (i, line) in nums.iter().enumerate() {
        for (start, n) in group_vector(&line) {
            println!("start: {}, n: {}", start, n);
            for j in start..start + n.len() {
                if possible_gears.contains(&(i, j)) {
                    println!("n: {}", n);
                    sum += n.parse::<u32>().unwrap();
                    break;
                }
            }
        }
    }
    println!("Part 1: {}", sum);
}
