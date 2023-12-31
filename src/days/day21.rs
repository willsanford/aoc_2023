use petgraph::algo::dijkstra;
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use std::{collections::HashMap, collections::HashSet, fs::read_to_string};

pub fn day21() {
    let filename = "data/day_21.txt";

    let binding = read_to_string(filename).unwrap();
    let mut start = (0, 0);

    let mut graph = UnGraph::<(), i32>::new_undirected();
    let mut node_indexes: HashMap<(i32, i32), NodeIndex> = HashMap::new();

    let m: HashMap<(i32, i32), char> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect::<Vec<Vec<((i32, i32), char)>>>()
        .iter()
        .flatten()
        .map(|x| *x)
        .collect();

    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for ((x, y), v) in m.clone() {
        if v == 'S' {
            start = (x, y);
        }
        if node_indexes.contains_key(&(x, y)) {
            continue;
        }
        let node = graph.add_node(());
        node_indexes.insert((x, y), node);
    }

    for ((x, y), v) in m.clone() {
        if v != '.' {
            println!("{:?}", v);
        }
        for (dx, dy) in dirs.clone() {
            if let Some(_) = m.get(&(x + dx, y + dy)) {
                graph.add_edge(
                    *node_indexes.get(&(x, y)).unwrap(),
                    *node_indexes.get(&(x + dx, y + dy)).unwrap(),
                    1,
                );
            }
        }
    }

    let mut distances = dijkstra(&graph, *node_indexes.get(&start).unwrap(), None, |e| {
        *e.weight()
    });

    println!("{:?}", distances.len());
    println!("{:?}", node_indexes.len());
    distances.insert(*node_indexes.get(&start).unwrap(), 0);

    let p1 = distances
        .values()
        .filter(|v| **v <= 64 && **v % 2 == 0)
        .count();
    println!("Part 1: {:?}", p1);

    let max_x = *(m.iter().map(|((x, _), _)| x).max().unwrap()) + 1;
    let n = ((26501365 - (max_x / 2)) / max_x) as usize;
    let even_corners = distances
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = distances
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    // Solution from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let ans = odd * distances.values().filter(|v| **v % 2 == 1).count()
        + even * distances.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    println!("Part 2: {:?}", ans);
}
/*
pub fn day21() {
    let filename = "data/day_21_ex.txt";

    let binding = read_to_string(filename).unwrap();
    let m: HashMap<(i32, i32), char> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect::<Vec<Vec<((i32, i32), char)>>>()
        .iter()
        .flatten()
        .map(|x| *x)
        .collect();

    let max_x = *(m.iter().map(|((x, _), _)| x).max().unwrap());
    let max_y = *(m.iter().map(|((_, y), _)| y).max().unwrap());

    let start: (i32, i32) = m
        .iter()
        .find_map(|(k, v)| if *v == 'S' { Some(*k) } else { None })
        .unwrap();

    let mut states: HashMap<(i32, i32), HashSet<[u32; 4]>> = HashMap::new(); // ((x, y), [n, s, e, w])
    states.insert(start, HashSet::from_iter(vec![[0; 4]]));

    let dirs = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for _round in 0..10 {
        let mut new_states: HashMap<(i32, i32), HashSet<[u32; 4]>> = HashMap::new();

        for ((x, y), v) in states {
            for (dx, dy) in dirs.clone() {
                if let Some(c) = m.get(&(x + dy, y + dy)) {
                    if *c == '.' || *c == 'S' {
                        if let Some(s_) = new_states.get_mut(&(x + dx, y + dy)) {
                            for v_ in v.clone() {
                                s_.insert(v_);
                            }
                        } else {
                            new_states.insert((x + dx, y + dy), v.clone());
                        }
                    }
                } else {
                    // Get the new set of states
                    let ((new_x, new_y), dir): ((i32, i32), usize) = match (dx, dy) {
                        (1, 0) => ((0, y), 2),
                        (-1, 0) => ((max_x, y), 3),
                        (0, 1) => ((x, 0), 1),
                        (0, -1) => ((x, max_y), 0),
                        _ => unreachable!(),
                    };

                    if let Some(c_) = m.get(&(new_x, new_y)) {
                        if *c_ == '.' || *c_ == 'S' {
                            // Check this direction
                            if let Some(s_) = new_states.get_mut(&(new_x, new_y)) {
                                for v_ in v.clone() {
                                    let mut new_v_ = v_;
                                    new_v_[dir] += 1;
                                    s_.insert(new_v_);
                                }
                            } else {
                                let mut new_v_ = HashSet::new();
                                for mut v_ in v.clone() {
                                    v_[dir] += 1;
                                    new_v_.insert(v_);
                                }
                                new_states.insert((x + dx, y + dy), new_v_);
                            }
                        }
                    }
                }
            }
        }
        states = new_states;
    }

    println!("{:?}", states.len());
}

pub fn _day21() {
    let filename = "data/day_21.txt";

    let binding = read_to_string(filename).unwrap();
    let m: HashMap<(i32, i32), char> = binding
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<Vec<((i32, i32), char)>>()
        })
        .collect::<Vec<Vec<((i32, i32), char)>>>()
        .iter()
        .flatten()
        .map(|x| *x)
        .collect();

    let start: (i32, i32) = m
        .iter()
        .find_map(|(k, v)| if *v == 'S' { Some(*k) } else { None })
        .unwrap();

    let mut states = HashSet::new();
    states.insert(start);

    for _round in 0..64 {
        let mut new_states = HashSet::new();

        for state in states {
            for (dx, dy) in vec![(1, 0), (-1, 0), (0, 1), (0, -1)] {
                if let Some(c) = m.get(&(state.0 + dx, state.1 + dy)) {
                    if *c == '.' || *c == 'S' {
                        new_states.insert((state.0 + dx, state.1 + dy));
                    }
                }
            }
        }
        states = new_states;
    }

    println!("{:?}", states.len());
}
*/
