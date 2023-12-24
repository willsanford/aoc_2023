use std::collections::HashMap;
use std::fs::read_to_string;

pub fn day23() {
    let filename = "data/day_23_ex.txt";

    let m: HashMap<(i32, i32), char> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let max_y = m.keys().map(|(_, y)| y).max().unwrap();

    let start = m
        .iter()
        .filter(|(k, v)| **v == '.' && k.1 == 0)
        .next()
        .unwrap()
        .0;

    let _end = m
        .iter()
        .filter(|(k, v)| **v == '.' && k.1 == *max_y)
        .next()
        .unwrap()
        .0;

    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut _spaces: HashMap<(i32, i32), (i32, Vec<i32>)> = HashMap::new(); // (x, y), dis, track_id

    let mut to_check: Vec<(i32, i32, i32, Vec<i32>)> = vec![(start.0, start.1, 0, vec![0])];

    while let Some((x, y, _dis, _track_id)) = to_check.pop() {
        let possible_spaces = dirs
            .iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(x, y)| m.get(&(*x, *y)).is_some() && *m.get(&(*x, *y)).unwrap() != '#')
            .collect::<Vec<_>>();

        for (_x, _y) in possible_spaces {
            // Check if this space is a hill
            // Check if we have already seen this space
            // If we have already seen this space, check the track_id
        }
        // If we are going multiple directions, split the track_id
    }
}
