use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn day11() {
    let filename = "data/day_11.txt";
    let binding = read_to_string(filename).unwrap();
    let data = binding.split('\n').collect_vec();
    let map_: Vec<Vec<(usize, usize)>> = data
        .iter()
        .filter(|line| !line.is_empty())
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(j, _)| (i, j))
                .collect()
        })
        .collect();
    let map: Vec<(usize, usize)> = map_.iter().flatten().copied().collect();

    let mut unused_rows: HashSet<usize> = (0..data.len()).collect::<HashSet<usize>>();
    let mut unused_cols: HashSet<usize> = (0..data[0].len()).collect::<HashSet<usize>>();

    map.iter().for_each(|(i, j)| {
        unused_rows.remove(i);
        unused_cols.remove(j);
    });

    let ans: i64 = map
        .iter()
        .cartesian_product(&map)
        .map(|((ax, ay), (bx, by))| {
            let dx = (*ax as i64 - *bx as i64).abs();
            let dy = (*ay as i64 - *by as i64).abs();

            let expanded_rows: HashSet<usize> = if ax < bx {
                (ax + 1..*bx).collect()
            } else {
                (bx + 1..*ax).collect()
            };

            let expanded_cols: HashSet<usize> = if ay < by {
                (ay + 1..*by).collect()
            } else {
                (by + 1..*ay).collect()
            };

            let dis = dx
                + dy
                + ((1e6 as i64 - 1) * unused_rows.intersection(&expanded_rows).count() as i64)
                + ((1e6 as i64 - 1) * unused_cols.intersection(&expanded_cols).count() as i64);
            dis
        })
        .sum();
    println!("{:?}", ans / 2);
}
