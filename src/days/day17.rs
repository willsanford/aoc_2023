use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
    S, // Still Direction for when we start
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Space {
    val: i32,
    dis: i32,
    pos: (i32, i32),
    bt: (i32, i32),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    pos: (i32, i32),
    last_dir: Dir,
}

impl State {
    fn successors(&self, m: &HashMap<(i32, i32), Space>) -> Vec<(State, usize)> {
        let mut next_states: Vec<(State, usize)> = Vec::new();

        let possible_dirs = match self.last_dir {
            Dir::U | Dir::D => vec![Dir::L, Dir::R],
            Dir::L | Dir::R => vec![Dir::U, Dir::D],
            Dir::S => vec![Dir::D, Dir::R],
        };

        for d in possible_dirs {
            // Go three clicks in any direction
            let mut cost = 0;
            let dir = match d {
                Dir::U => (0, -1),
                Dir::D => (0, 1),
                Dir::L => (-1, 0),
                Dir::R => (1, 0),
                _ => unreachable!(),
            };

            for i in 1..=10 {
                // Part 1: 1..=3
                let next_pos = (self.pos.0 + dir.0 * i, self.pos.1 + dir.1 * i);
                if let Some(s) = m.get(&next_pos) {
                    cost += s.val as usize;
                    if i >= 4 {
                        // Part 1 no if
                        next_states.push((
                            State {
                                pos: next_pos,
                                last_dir: d,
                            },
                            cost,
                        ));
                    }
                }
            }
        }
        next_states
    }
}

pub fn day17() {
    let filename = "data/day_17.txt";

    let mut spaces: HashMap<(i32, i32), Space> = HashMap::new();
    read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                spaces.insert(
                    (x as i32, y as i32),
                    Space {
                        val: c as i32 - '0' as i32,
                        dis: i32::MAX,
                        pos: (x as i32, y as i32),
                        bt: (0, 0),
                    },
                );
            });
        });

    // Find the end
    let max_x = spaces.keys().map(|(x, _)| x).max().unwrap();
    let max_y = spaces.keys().map(|(_, y)| y).max().unwrap();

    let goal: (i32, i32) = (*max_x, *max_y);
    let start: State = State {
        pos: (0, 0),
        last_dir: Dir::S,
    };
    let result = dijkstra(&start, |p| p.successors(&spaces), |p| p.pos == goal);
    println!("Result: {:?}", result.unwrap().1);
}
