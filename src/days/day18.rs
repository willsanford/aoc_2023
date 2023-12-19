use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

// https://cp-algorithms.com/geometry/area-of-simple-polygon.html
fn shoelace_area(coords: &Vec<(i64, i64)>) -> i64 {
    let mut area = 0;

    for i in 0..coords.len() {
        let p = if let Some(i_) = i.checked_sub(1) {
            coords[i_]
        } else {
            coords[coords.len() - 1]
        };
        let q = coords[i];

        // Add the area from the interior
        area += (p.1 - q.1) * (p.0 + q.0);

        // Add the area of the border tiles
        area += (p.0 - q.0).abs();
        area += (p.1 - q.1).abs();
    }
    area.abs() / 2 + 1
}
pub fn day18() {
    let filename = "data/day_18.txt";

    let binding = read_to_string(filename).unwrap();
    let coords: Vec<(i64, i64)> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .into_iter()
        .scan((0i64, 0i64), |state, line| {
            let line_s = line.split(' ').collect::<Vec<&str>>()[2];
            let mut l: String = line_s.to_string();
            l.remove(0);
            l.remove(0);
            l.remove(6);

            let (left, right) = l.split_at(5);
            let dir = match right {
                "0" => (0, 1),
                "1" => (1, 0),
                "2" => (0, -1),
                "3" => (-1, 0),
                _ => panic!("Invalid operation"),
            };

            let dis = i64::from_str_radix(left, 16).unwrap();

            state.0 += dir.0 * dis;
            state.1 += dir.1 * dis;
            Some(*state)
        })
        .collect();
    let enclosed_area = shoelace_area(&coords);

    println!("{:?}", enclosed_area);
}

pub fn _day18() {
    let filename = "data/day_18.txt";

    let binding = read_to_string(filename).unwrap();
    let opps: Vec<(Dir, i32, &str)> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .into_iter()
        .map(|line| {
            let line_s = line.split(' ').collect::<Vec<&str>>();
            let dir = match line_s[0] {
                "U" => Dir::U,
                "D" => Dir::D,
                "L" => Dir::L,
                "R" => Dir::R,
                _ => panic!("Invalid direction"),
            };
            let dist: i32 = line_s[1].parse::<i32>().unwrap();
            (dir, dist, line_s[2])
        })
        .collect();

    let mut pos = (0, 0);
    let mut walls: Vec<(i32, i32, Dir, &str)> = Vec::new();

    for (dir, dist, c) in opps.iter() {
        let change = match dir {
            Dir::U => (0, -1),
            Dir::D => (0, 1),
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
        };

        for _ in 0..*dist {
            walls.push((pos.0, pos.1, *dir, c));
            pos.0 += change.0;
            pos.1 += change.1;
        }
    }

    let m: HashMap<(i32, i32), (Dir, &str)> = walls
        .iter()
        .map(|(x, y, dir, c)| ((*x, *y), (*dir, *c)))
        .collect();

    // get max x and y
    let max_x = walls.iter().map(|(x, _, _, _)| x).max().unwrap();
    let max_y = walls.iter().map(|(_, y, _, _)| y).max().unwrap();

    // get min x and y
    let min_x = walls.iter().map(|(x, _, _, _)| x).min().unwrap();
    let min_y = walls.iter().map(|(_, y, _, _)| y).min().unwrap();

    let mut ans: u32 = 0;

    // Get whether any point is within the polygon
    for x in *min_x..=*max_x {
        for y in *min_y..=*max_y {
            if m.contains_key(&(x, y)) {
                continue;
            }

            let mut collisions = 0;

            let mut dirs_on_wall: Vec<Dir> = Vec::new();
            let mut curr_x = x;
            while curr_x <= *max_x + 1 {
                if let Some((_dir, _c)) = m.get(&(curr_x, y)) {
                    let i: usize = walls
                        .iter()
                        .position(|(x_, y_, _, _)| *x_ == curr_x && *y_ == y)
                        .unwrap();
                    // Check the position before and after and see if either is coming up or down
                    let i_p = i.checked_sub(1).unwrap_or(walls.len() - 1);
                    let i_n = if i + 1 < walls.len() { i + 1 } else { 0 };
                    let (_, y_p, _, _) = walls[i_p];
                    let (_, y_n, _, _) = walls[i_n];

                    // Get whether the previous or next point is above or below this point
                    match (y_p < y, y_p > y, y_n < y, y_n > y) {
                        (true, false, false, true) => {
                            dirs_on_wall.push(Dir::D);
                            dirs_on_wall.push(Dir::D);
                        }
                        (false, true, true, false) => {
                            dirs_on_wall.push(Dir::U);
                            dirs_on_wall.push(Dir::U);
                        }
                        (false, true, _, _) => {
                            dirs_on_wall.push(Dir::U);
                        }
                        (true, false, _, _) => {
                            dirs_on_wall.push(Dir::D);
                        }
                        (_, _, false, true) => {
                            dirs_on_wall.push(Dir::D);
                        }
                        (_, _, true, false) => {
                            dirs_on_wall.push(Dir::U);
                        }
                        (false, false, false, false) => {}
                        _ => {}
                    }

                    if dirs_on_wall.len() == 2 {
                        // Check if this is a collision
                        if dirs_on_wall[0] == dirs_on_wall[1] {
                            collisions += 1;
                        }
                        dirs_on_wall = Vec::new();
                    }
                }
                curr_x += 1;
            }
            if collisions % 2 == 1 {
                ans += 1;
            }
        }
    }

    println!("{:?}", ans + walls.len() as u32);
}
