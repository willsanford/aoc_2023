use std::collections::HashSet;
use std::fs::read_to_string;

fn map_dir(c: char, d: (i32, i32)) -> (i32, i32) {
    match (c, d) {
        ('|', _) => d, // Do nothing, keep moving
        ('-', _) => d, // Do nothing, keep moving
        ('L', (1, 0)) => (0, 1),
        ('L', (0, -1)) => (-1, 0),
        ('J', (0, 1)) => (-1, 0),
        ('J', (1, 0)) => (0, -1),
        ('F', (-1, 0)) => (0, 1),
        ('F', (0, -1)) => (1, 0),
        ('7', (0, 1)) => (1, 0),
        ('7', (-1, 0)) => (0, -1),
        _ => unreachable!(),
    }
}

fn get_start_char(dirs: HashSet<(i32, i32)>) -> char {
    if dirs.contains(&(-1, 0)) && dirs.contains(&(0, 1)) {
        'L'
    } else if dirs.contains(&(0, -1)) && dirs.contains(&(-1, 0)) {
        'J'
    } else if dirs.contains(&(1, 0)) && dirs.contains(&(0, 1)) {
        'F'
    } else if dirs.contains(&(0, -1)) && dirs.contains(&(1, 0)) {
        '7'
    } else if dirs.contains(&(1, 0)) && dirs.contains(&(-1, 0)) {
        '|'
    } else if dirs.contains(&(0, -1)) && dirs.contains(&(0, 1)) {
        '-'
    } else {
        unreachable!()
    }
}

pub fn day10() {
    let filename = "data/day_10.txt";

    let mut map: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    // Find the starting points
    let mut starting_pos: (usize, usize) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                starting_pos = (i, j);
            }
        }
    }

    // Dermine the corrent way to start
    let mut direction: (i32, i32) = (0, 0);
    let mut dirs: HashSet<(i32, i32)> = HashSet::new();
    if starting_pos.0 > 0
        && (map[starting_pos.0.checked_sub(1).unwrap()][starting_pos.1] == '7'
            || map[starting_pos.0.checked_sub(1).unwrap()][starting_pos.1] == '|'
            || map[starting_pos.0.checked_sub(1).unwrap()][starting_pos.1] == 'F')
    {
        dirs.insert((-1, 0));
        direction = (-1, 0);
    }
    if starting_pos.0 < map.len() - 1
        && ((map[starting_pos.0 + 1][starting_pos.1] == '|')
            || (map[starting_pos.0 + 1][starting_pos.1] == 'L')
            || (map[starting_pos.0 + 1][starting_pos.1] == 'J'))
    {
        dirs.insert((1, 0));
        direction = (1, 0);
    }
    if starting_pos.1 < map[0].len() - 1
        && ((map[starting_pos.0][starting_pos.1 + 1] == '-')
            || (map[starting_pos.0][starting_pos.1 + 1] == 'J')
            || (map[starting_pos.0][starting_pos.1 + 1] == '7'))
    {
        dirs.insert((0, 1));
        direction = (0, 1);
    }
    if starting_pos.1 > 0
        && (map[starting_pos.0][starting_pos.1.checked_sub(1).unwrap()] == '-'
            || map[starting_pos.0][starting_pos.1.checked_sub(1).unwrap()] == 'F'
            || map[starting_pos.0][starting_pos.1.checked_sub(1).unwrap()] == 'L')
    {
        dirs.insert((0, -1));
        direction = (0, -1);
    }

    let start_char = get_start_char(dirs);
    map[starting_pos.0][starting_pos.1] = start_char;

    let mut current_pos: (usize, usize) = starting_pos;
    let mut seen = HashSet::new();

    loop {
        seen.insert(current_pos);
        // update position
        current_pos.0 = (current_pos.0 as i32 + direction.0) as usize;
        current_pos.1 = (current_pos.1 as i32 + direction.1) as usize;
        if current_pos == starting_pos {
            break;
        }
        direction = map_dir(map[current_pos.0][current_pos.1], direction);
    }

    let ans: i32 = map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, _c)| {
                    if seen.contains(&(i, j)) {
                        0
                    } else {
                        // Run from this point to the edge of the map.
                        let mut n: i32 = 0;
                        let mut seg_start: char = 'A';
                        for j_ in j..line.len() {
                            if seen.contains(&(i, j_)) {
                                // chars that map to getting on the pipe
                                n = match map[i][j_] {
                                    '|' => n + 1,
                                    '-' => n, // Do nothing
                                    'L' | 'F' => {
                                        seg_start = map[i][j_];
                                        n
                                    }
                                    'J' => {
                                        if seg_start == 'F' {
                                            n + 1
                                        } else {
                                            n
                                        }
                                    }
                                    '7' => {
                                        if seg_start == 'L' {
                                            n + 1
                                        } else {
                                            n
                                        }
                                    }

                                    _ => unreachable!(),
                                }
                            }
                        }
                        if n % 2 == 0 || n == 0 {
                            0
                        } else {
                            1
                        }
                    }
                })
                .sum::<i32>()
        })
        .sum();
    println!("{:?}", ans);
}

pub fn _day10() {
    let filename = "data/day_10.txt";

    let map: Vec<Vec<char>> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    // Find the starting points
    let mut starting_pos: (usize, usize) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                starting_pos = (i, j);
            }
        }
    }

    // Dermine the corrent way to start
    let mut direction: (i32, i32) = (0, 0);
    if starting_pos.0 > 0
        && (map[starting_pos.0.checked_sub(1).unwrap()][starting_pos.1] == '7'
            || map[starting_pos.0.checked_sub(1).unwrap()][starting_pos.1] == '|'
            || map[starting_pos.0.checked_sub(1).unwrap()][starting_pos.1] == 'F')
    {
        direction.0 = -1;
    } else if starting_pos.0 < map.len() - 1
        && ((map[starting_pos.0 + 1][starting_pos.1] == '|')
            || (map[starting_pos.0 + 1][starting_pos.1] == '#')
            || (map[starting_pos.0 + 1][starting_pos.1] == '#'))
    {
        direction.0 = 1;
    } else if starting_pos.1 < map[0].len() - 1
        && ((map[starting_pos.0][starting_pos.1 + 1] == '-')
            || (map[starting_pos.0][starting_pos.1 + 1] == 'J')
            || (map[starting_pos.0][starting_pos.1 + 1] == '7'))
    {
        direction.1 = 1;
    } else if starting_pos.1 > 0
        && (map[starting_pos.0][starting_pos.1.checked_sub(1).unwrap()] == '7'
            || map[starting_pos.0][starting_pos.1.checked_sub(1).unwrap()] == '|'
            || map[starting_pos.0][starting_pos.1.checked_sub(1).unwrap()] == 'F')
    {
        direction.1 = -1;
    }
    println!("{:?}", direction);

    let mut current_pos: (usize, usize) = starting_pos;

    // continue iterate until we find the start again

    let mut ans = 0;
    loop {
        println!("{:?}", current_pos);
        // update position
        current_pos.0 = (current_pos.0 as i32 + direction.0) as usize;
        current_pos.1 = (current_pos.1 as i32 + direction.1) as usize;
        if current_pos == starting_pos {
            break;
        }
        direction = map_dir(map[current_pos.0][current_pos.1], direction);
    }
    ans = (ans as f32 / 2.0).ceil() as i32;
    println!("{:?}", starting_pos);

    println!("{:?}", ans);
}
