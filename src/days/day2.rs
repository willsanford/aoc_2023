use std::cmp::max;
use std::fs::read_to_string;

fn parse_set(set: &str) -> [u32; 3] {
    let mut ans: [u32; 3] = [0, 0, 0];
    for el in set.split(',') {
        let color = el.trim().split(' ').collect::<Vec<&str>>();
        match color[1] {
            "red" => ans[0] = color[0].parse().unwrap(),
            "green" => ans[1] = color[0].parse().unwrap(),
            "blue" => ans[2] = color[0].parse().unwrap(),
            _ => (),
        }
    }
    return ans;
}

#[allow(dead_code)]
fn part2_fn<I>(iter: I) -> u32
where
    I: Iterator<Item = [u32; 3]>,
{
    let i = iter.fold([0, 0, 0], |mut acc, set| {
        acc[0] = max(acc[0], set[0]);
        acc[1] = max(acc[1], set[1]);
        acc[2] = max(acc[2], set[2]);
        acc
    });
    i[0] * i[1] * i[2]
}

#[allow(dead_code)]
fn part1_fn<I>(mut iter: I, game_num: u32) -> u32
where
    I: Iterator<Item = [u32; 3]>,
{
    let maxes: [u32; 3] = [12, 13, 14];
    if iter
        .find_map(|set| {
            if set[0] > maxes[0] || set[1] > maxes[1] || set[2] > maxes[2] {
                Some(1)
            } else {
                None
            }
        })
        .is_none()
    {
        game_num
    } else {
        0
    }
}

pub fn day2() {
    let filename = "data/day_2.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .trim()
        .split('\n')
        .into_iter()
        .map(|line| {
            let game = line.split(':').collect::<Vec<&str>>();
            #[allow(unused_variables)]
            let game_num: u32 = game[0].split(' ').collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();

            // Get the sets
            let valid_set = game[1].split(';').map(parse_set);
            // part1_fn(valid_set, game_num)
            part2_fn(valid_set)
        })
        .sum();

    println!("{:?}", ans);
}
