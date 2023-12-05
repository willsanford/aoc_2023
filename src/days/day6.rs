use std::fs::read_to_string;

fn get_times(time: u64, dist: u64) -> u64 {
    for i in 1..time {
        if i * (time - i) > dist {
            return time - (2 * i) + 1;
        }
    }
    return 0; // unreachable
}

pub fn day6() {
    let filename = "data/day_6.txt";
    let binding = read_to_string(filename).unwrap();
    let arrs = binding
        .split('\n')
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|line| {
            if line.len() < 2 {
                return Err("Invalid input");
            }
            Ok(line[1]
                .trim()
                .chars()
                .filter(|x| !x.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
                .unwrap())
        })
        .filter_map(Result::ok)
        .collect::<Vec<u64>>();

    println!("{:?}", get_times(arrs[0], arrs[1]));
}
pub fn _day6() {
    let filename = "data/day_6.txt";
    let binding = read_to_string(filename).unwrap();
    let arrs = binding
        .split('\n')
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .filter(|line| line.len() > 1)
        .map(|line| {
            line[1]
                .trim()
                .split(' ')
                .map(|x| x.trim().parse::<u64>())
                .filter_map(Result::ok)
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let out: u64 = arrs[0]
        .iter()
        .zip(arrs[1].iter())
        .map(|(time, dist)| get_times(*time, *dist))
        .product();

    println!("{:?}", out);
}
