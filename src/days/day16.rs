use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug)]
struct Space {
    dirs: [bool; 4], // N, E, S, W
    c: char,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    L,
    R,
    U,
    D,
}

pub fn day16() {
    let filename = "data/day_16.txt";

    let mut spaces: HashMap<(i32, i32), Space> = HashMap::new();
    let binding = read_to_string(filename).unwrap();
    binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                spaces.insert(
                    (i as i32, j as i32),
                    Space {
                        dirs: [false; 4],
                        c,
                    },
                );
            })
        });

    let max_y = spaces.keys().map(|(y, _)| y).max().unwrap();
    let max_x = spaces.keys().map(|(_, x)| x).max().unwrap();

    // generate all the starting points and their directions
    let mut starting_directions = vec![];
    for i in 0..=*max_x {
        starting_directions.push(((0, i), Dir::D));
        starting_directions.push(((*max_y, i), Dir::U));
    }
    for j in 0..=*max_y {
        starting_directions.push(((j, 0), Dir::R));
        starting_directions.push(((j, *max_x), Dir::L));
    }

    let ans: u32 = starting_directions
        .iter()
        .map(|start| {
            // Reset the spaces map
            spaces
                .iter_mut()
                .for_each(|(_, space)| space.dirs = [false; 4]);
            let mut to_check: HashSet<((i32, i32), Dir)> = HashSet::new();
            to_check.insert(*start);

            while let Some(el) = to_check.iter().next().cloned() {
                to_check.remove(&el);
                let ((i, j), dir) = el;
                let space = spaces.get_mut(&(i, j)).unwrap();
                space.dirs[dir as usize] = true;

                use Dir::*;
                let new_dirs: Vec<((i32, i32), Dir)> = match (space.c, dir) {
                    ('.', D) => {
                        vec![((i + 1, j), D)]
                    }
                    ('.', U) => {
                        vec![((i - 1, j), U)]
                    }
                    ('.', L) => {
                        vec![((i, j - 1), L)]
                    }
                    ('.', R) => {
                        vec![((i, j + 1), R)]
                    }
                    ('-', R) => {
                        vec![((i, j + 1), R)]
                    }
                    ('-', L) => {
                        vec![((i, j - 1), L)]
                    }
                    ('-', U) | ('-', D) => {
                        vec![((i, j - 1), L), ((i, j + 1), R)]
                    }
                    ('|', R) | ('|', L) => {
                        vec![((i + 1, j), D), ((i - 1, j), U)]
                    }
                    ('|', U) => {
                        vec![((i - 1, j), U)]
                    }
                    ('|', D) => {
                        vec![((i + 1, j), D)]
                    }
                    ('\\', D) => {
                        vec![((i, j + 1), R)]
                    }
                    ('\\', U) => {
                        vec![((i, j - 1), L)]
                    }
                    ('\\', L) => {
                        vec![((i - 1, j), U)]
                    }
                    ('\\', R) => {
                        vec![((i + 1, j), D)]
                    }
                    ('/', D) => {
                        vec![((i, j - 1), L)]
                    }
                    ('/', U) => {
                        vec![((i, j + 1), R)]
                    }
                    ('/', L) => {
                        vec![((i + 1, j), D)]
                    }
                    ('/', R) => {
                        vec![((i - 1, j), U)]
                    }
                    _ => {
                        unreachable!()
                    }
                };

                for dir in new_dirs {
                    // Make sure we have a valid space and we havnt checked this direction yet
                    if let Some(space) = spaces.get_mut(&dir.0) {
                        if !space.dirs[dir.1 as usize] {
                            to_check.insert(dir);
                        }
                    }
                }
            }

            let ans_ = spaces
                .iter()
                .filter(|(_, space)| space.dirs.iter().any(|x| *x))
                .count() as u32;
            ans_
        })
        .max()
        .unwrap();

    println!("{:?}", ans);
}

pub fn _day16() {
    let filename = "data/day_16.txt";

    let mut spaces: HashMap<(i32, i32), Space> = HashMap::new();
    let binding = read_to_string(filename).unwrap();
    binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                spaces.insert(
                    (i as i32, j as i32),
                    Space {
                        dirs: [false; 4],
                        c,
                    },
                );
            })
        });

    let mut to_check: HashSet<((i32, i32), Dir)> = HashSet::new();
    to_check.insert(((0, 0), Dir::R));

    while let Some(el) = to_check.iter().next().cloned() {
        to_check.remove(&el);
        let ((i, j), dir) = el;
        let space = spaces.get_mut(&(i, j)).unwrap();
        space.dirs[dir as usize] = true;

        use Dir::*;
        let new_dirs: Vec<((i32, i32), Dir)> = match (space.c, dir) {
            ('.', D) => {
                vec![((i + 1, j), D)]
            }
            ('.', U) => {
                vec![((i - 1, j), U)]
            }
            ('.', L) => {
                vec![((i, j - 1), L)]
            }
            ('.', R) => {
                vec![((i, j + 1), R)]
            }
            ('-', R) => {
                vec![((i, j + 1), R)]
            }
            ('-', L) => {
                vec![((i, j - 1), L)]
            }
            ('-', U) | ('-', D) => {
                vec![((i, j - 1), L), ((i, j + 1), R)]
            }
            ('|', R) | ('|', L) => {
                vec![((i + 1, j), D), ((i - 1, j), U)]
            }
            ('|', U) => {
                vec![((i - 1, j), U)]
            }
            ('|', D) => {
                vec![((i + 1, j), D)]
            }
            ('\\', D) => {
                vec![((i, j + 1), R)]
            }
            ('\\', U) => {
                vec![((i, j - 1), L)]
            }
            ('\\', L) => {
                vec![((i - 1, j), U)]
            }
            ('\\', R) => {
                vec![((i + 1, j), D)]
            }
            ('/', D) => {
                vec![((i, j - 1), L)]
            }
            ('/', U) => {
                vec![((i, j + 1), R)]
            }
            ('/', L) => {
                vec![((i + 1, j), D)]
            }
            ('/', R) => {
                vec![((i - 1, j), U)]
            }
            _ => {
                unreachable!()
            }
        };

        for dir in new_dirs {
            // Make sure we have a valid space and we havnt checked this direction yet
            if let Some(space) = spaces.get_mut(&dir.0) {
                if !space.dirs[dir.1 as usize] {
                    to_check.insert(dir);
                }
            }
        }
    }

    let ans: u32 = spaces
        .iter()
        .filter(|(_, space)| space.dirs.iter().any(|x| *x))
        .count() as u32;

    println!("{:?}", ans);
}
