use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

fn part_1(p1: &Pos, p2: &Pos) -> bool {
    let min = 200000000000000i64;
    let max = 400000000000000i64;
    if p1.dx * p2.dy == p2.dx * p1.dy {
        return false;
    }

    let t1 = (p2.dy * (p1.x - p2.x) - p2.dx * (p1.y - p2.y)) / (p1.dy * p2.dx - p1.dx * p2.dy);
    let t2 = (p1.dy * (p2.x - p1.x) - p1.dx * (p2.y - p1.y)) / (p2.dy * p1.dx - p2.dx * p1.dy);
    if t1 < 0 || t2 < 0 {
        return false;
    }
    let ix = p1.x + t1 * p1.dx;
    let iy = p1.y + t1 * p1.dy;

    if min <= ix && ix <= max && min <= iy && iy <= max {
        return true;
    }

    false
}

pub fn day24() {
    let filename = "data/day_24.txt";

    let p: Vec<Pos> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once('@').unwrap();
            let vals_right = right
                .split(',')
                .filter(|s| s.len() > 0)
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let vals_left = left
                .split(',')
                .filter(|s| s.len() > 0)
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            Pos {
                x: vals_left[0],
                y: vals_left[1],
                z: vals_left[2],
                dx: vals_right[0],
                dy: vals_right[1],
                dz: vals_right[2],
            }
        })
        .collect();
    let ans = p
        .iter()
        .cartesian_product(p.clone())
        .filter(|(a, b)| **a != *b && part_1(a, b))
        .count();
    println!("{:?}", ans / 2);
}
