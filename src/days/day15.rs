use std::fs::read_to_string;

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

pub fn day15() {
    let filename = "data/day_15.txt";

    let binding = read_to_string(filename).unwrap();
    let opps: Vec<(usize, &str, bool, u32)> = binding
        .split(',')
        .into_iter()
        .map(|x| {
            let (val, lens) = x.split_once(['-', '=']).unwrap();
            let add = x.contains('=');
            (
                hash(val) as usize,
                val,
                add,
                if add { lens.parse().unwrap() } else { 0 },
            )
        })
        .collect();

    let boxes: Vec<Vec<(&str, u32)>> = opps.iter().fold(
        vec![vec![]; 256],
        |mut acc, (box_label, label, add, lens)| {
            if *add {
                // Label is not already in the box
                if let Some(ind) = acc[*box_label].iter().position(|(l, _)| *l == *label) {
                    acc[*box_label][ind] = (*label, *lens);
                } else {
                    acc[*box_label].push((*label, *lens));
                }
            } else {
                if let Some(ind) = acc[*box_label].iter().position(|(l, _)| *l == *label) {
                    acc[*box_label].remove(ind);
                }
                // If this does not succeed, then do not do anything
            }
            acc
        },
    );

    let ans: u32 = boxes.iter().enumerate().fold(0, |acc, (i, v)| {
        acc + v.iter().enumerate().fold(0, |acc_, (j, (_, val))| {
            acc_ + (i as u32 + 1) * (j as u32 + 1) * val
        })
    });
    println!("{:?}", ans);
}
pub fn _day15() {
    let filename = "data/day_15.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .split(',')
        .into_iter()
        .map(hash)
        .sum();

    println!("{:?}", ans);
}
