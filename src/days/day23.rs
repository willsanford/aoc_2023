use petgraph::algo::{all_simple_paths, dijkstra};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, Graph, NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

const PART1: bool = false;

pub fn day23() {
    let filename = "data/day_23.txt";

    let m: HashMap<(i32, i32), char> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
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

    let end = m
        .iter()
        .filter(|(k, v)| **v == '.' && k.1 == *max_y)
        .next()
        .unwrap()
        .0;

    let mut g: Graph<(i32, i32), i32, Undirected> = UnGraph::new_undirected();

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Get all the edges in the graph
    let edges: Vec<((i32, i32), (i32, i32), i32)> = m
        .iter()
        .map(|(k, v)| {
            // Check all the cardinal directions and the
            dirs.iter()
                .filter(|(x, y)| m.contains_key(&(k.0 + *x, k.1 + *y)))
                .map(|(x, y)| (*k, (k.0 + *x, k.1 + *y), 1))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut node_indices = HashMap::new();
    for &(start, end, weight) in &edges {
        // Dont double count edges
        if node_indices.contains_key(&start) && node_indices.contains_key(&end) {
            if g.contains_edge(node_indices[&start], node_indices[&end]) {
                continue;
            }
        }
        let start_index = *node_indices
            .entry(start)
            .or_insert_with(|| g.add_node(start));
        let end_index = *node_indices.entry(end).or_insert_with(|| g.add_node(end));

        g.add_edge(start_index, end_index, weight);
    }

    let mut critical_vertices: HashSet<_> = g
        .node_indices()
        .filter(|&x| {
            g.neighbors_directed(x, petgraph::Direction::Outgoing)
                .count()
                > 2
        })
        .collect();
    critical_vertices.insert(node_indices[start]);
    critical_vertices.insert(node_indices[end]);

    // edges between critical vertice
    let mut new_graph: Graph<(i32, i32), i32, Undirected> = Graph::new_undirected();
    let mut new_edges: Vec<((i32, i32), (i32, i32), i32)> = vec![];
    for vert in critical_vertices.clone() {
        // Get the out degree of the vertice
        let out_degree = g.neighbors_undirected(vert).count();

        let res = dijkstra(&g, vert, None, |e| {
            if critical_vertices.clone().contains(&e.source()) && &e.source() != &vert {
                i16::MAX as i32
            } else {
                *e.weight()
            }
        });
        let mut o_c: Vec<_> = res
            .iter()
            .filter(|(k, _)| **k != vert && critical_vertices.clone().contains(k))
            .collect();

        o_c.sort_by(|a, b| a.1.cmp(b.1));
        // Make an edge from this ciritical vertice to the three closest critical vertices
        for (i, (k, w)) in o_c.iter().enumerate() {
            if i == out_degree {
                break;
            }
            new_edges.push((g[vert], g[**k], **w));
        }
    }

    // Make the new graph
    let mut node_indices = HashMap::new();
    for &(start, end, weight) in &new_edges {
        // Dont double count edges
        if node_indices.contains_key(&start) && node_indices.contains_key(&end) {
            if new_graph.contains_edge(node_indices[&start], node_indices[&end]) {
                continue;
            }
        }
        let start_index = *node_indices
            .entry(start)
            .or_insert_with(|| new_graph.add_node(start));
        let end_index = *node_indices
            .entry(end)
            .or_insert_with(|| new_graph.add_node(end));

        new_graph.add_edge(start_index, end_index, weight);
    }

    println!("{:?}", new_graph.node_count());

    let ans =
        all_simple_paths::<Vec<_>, _>(&new_graph, node_indices[start], node_indices[end], 0, None)
            .map(|x| {
                // Get the total weight of the path
                x.windows(2)
                    .map(|y| {
                        let (a, b) = (y[0], y[1]);
                        let edge = new_graph.find_edge(a, b).unwrap();
                        *new_graph.edge_weight(edge).unwrap()
                    })
                    .sum::<i32>()
            })
            .max()
            .unwrap();
    println!("{:?}", ans);
    println!("{:?}", Dot::with_config(&new_graph, &[Config::EdgeNoLabel]));
}

pub fn _day23() {
    let filename = "data/day_23_ex.txt";

    let m: HashMap<(i32, i32), char> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '#')
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

    let end = m
        .iter()
        .filter(|(k, v)| **v == '.' && k.1 == *max_y)
        .next()
        .unwrap()
        .0;

    let mut g: DiGraph<(i32, i32), i32> = DiGraph::new();

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Get all the edges in the graph
    let edges: Vec<((i32, i32), (i32, i32), i32)> = m
        .iter()
        .map(|(k, v)| {
            // Check all the cardinal directions and the
            dirs.iter()
                .filter(|(x, y)| {
                    if let Some(c) = m.get(&(k.0 + *x, k.1 + *y)) {
                        match (v, c, (*x, *y)) {
                            ('.', '.', _) => true,
                            ('.', '<', (-1, 0)) => true,
                            ('<', '.', (-1, 0)) => true,
                            ('.', '>', (1, 0)) => true,
                            ('>', '.', (1, 0)) => true,
                            ('.', '^', (0, -1)) => true,
                            ('^', '.', (0, -1)) => true,
                            ('.', 'v', (0, 1)) => true,
                            ('v', '.', (0, 1)) => true,
                            ('<', '<', _) => true,
                            ('^', '^', _) => true,
                            ('v', 'v', _) => true,
                            ('>', '>', _) => true,
                            _ => false,
                        }
                    } else {
                        false
                    }
                })
                .map(|(x, y)| (*k, (k.0 + *x, k.1 + *y), 1))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let mut node_indices = HashMap::new();
    for &(start, end, weight) in &edges {
        let start_index = *node_indices
            .entry(start)
            .or_insert_with(|| g.add_node(start));
        let end_index = *node_indices.entry(end).or_insert_with(|| g.add_node(end));

        g.add_edge(start_index, end_index, weight);
    }

    let ans = all_simple_paths::<Vec<_>, _>(&g, node_indices[start], node_indices[end], 0, None)
        .map(|x| x.len())
        .max()
        .unwrap();
    println!("{:?}", ans - 1);
}
