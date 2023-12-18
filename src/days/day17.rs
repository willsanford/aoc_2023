use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Space {
    val: i32,
    dis: i32,
    pos: (i32, i32),
    bt: (i32, i32),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct QState {
    pos: (i32, i32),
    dir: Dir,
    streak: u32,
    cost: i32,
}

impl Ord for QState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

pub fn day17() {
    let filename = "data/day_17_ex.txt";

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

    // Find the start
    let max_x = spaces.keys().map(|(x, _)| x).max().unwrap();
    let max_y = spaces.keys().map(|(_, y)| y).max().unwrap();
    let end = (*max_x, *max_y);

    // change the first to have a distance of 0

    let mut to_visit: BinaryHeap<QState> = BinaryHeap::new();

    to_visit.push(QState {
        pos: (0, 0),
        dir: Dir::D,
        streak: 0,
        cost: spaces.get(&(0, 0)).unwrap().val,
    });

    let mut end_state: QState = QState {
        pos: (0, 0),
        dir: Dir::D,
        streak: 0,
        cost: 0,
    };
    'outer: while let Some(s) = to_visit.pop() {
        println!("{:?}", s);
        // Get the directions that we can travel from this node
        let mut dirs: Vec<Dir> = match s.dir {
            Dir::U | Dir::D => vec![Dir::L, Dir::R],
            Dir::L | Dir::R => vec![Dir::U, Dir::D],
        };
        // Add the current direction if we can continue
        if s.streak < 3 {
            dirs.push(s.dir);
        }

        // Check if they are better than our current state
        for dir in dirs {
            let coord = match dir {
                Dir::U => (s.pos.0, s.pos.1 - 1),
                Dir::D => (s.pos.0, s.pos.1 + 1),
                Dir::L => (s.pos.0 - 1, s.pos.1),
                Dir::R => (s.pos.0 + 1, s.pos.1),
            };
            if let Some(c) = spaces.get(&coord) {
                let new_state = QState {
                    pos: coord,
                    dir,
                    streak: if dir == s.dir { s.streak + 1 } else { 1 },
                    cost: s.cost + c.val,
                };

                if spaces.get(&coord).unwrap().dis > new_state.cost {
                    spaces.get_mut(&coord).unwrap().dis = new_state.cost;
                    spaces.get_mut(&coord).unwrap().bt = s.pos;
                    to_visit.push(new_state);
                }
                if new_state.pos == end {
                    end_state = new_state;
                    break 'outer;
                }
            }
        }
    }
    let mut current = end_state.pos;
    while current != (0, 0) {
        println!("{:?}", current);
        current = spaces.get(&current).unwrap().bt;
    }
    println!("Part 1: {}", end_state.cost);
}
