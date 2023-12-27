use petgraph::algo::{astar, tarjan_scc};
use petgraph::graph::UnGraph;
use petgraph::prelude::EdgeIndex;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn day25() {
    let filename = "data/day_25.txt";

    let edges: Vec<(String, String)> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .into_iter()
        .map(|line| {
            let (start, ends) = line.split_once(":").unwrap();
            ends.split(' ')
                .filter(|s| !s.is_empty())
                .map(|end| (start.to_string(), end.to_string()))
                .collect::<Vec<(String, String)>>()
        })
        .flatten()
        .collect();

    let mut graph = UnGraph::<String, i32>::new_undirected();

    let mut node_index = HashMap::new();
    for (start, end) in &edges {
        if !node_index.contains_key(start) {
            node_index.insert(start, graph.add_node(start.clone()));
        }
        if !node_index.contains_key(end) {
            node_index.insert(end, graph.add_node(end.clone()));
        }
        graph.add_edge(node_index[start], node_index[end], 1);
    }

    let unique_nodes: Vec<_> = node_index.iter().map(|(_, v)| *v).collect();
    let mut seen_edges: HashMap<EdgeIndex, u32> = HashMap::new();
    for _ in 0..300 {
        // Get two random nodes from the node list
        let start = unique_nodes.choose(&mut rand::thread_rng()).unwrap();
        let end = unique_nodes.choose(&mut rand::thread_rng()).unwrap();

        let path = astar(&graph, *start, |finish| finish == *end, |_| 1, |_| 0);

        for edge in path.unwrap().1.windows(2) {
            let edge_ref = graph.find_edge(edge[0], edge[1]).unwrap();
            if seen_edges.contains_key(&edge_ref) {
                seen_edges.insert(edge_ref, seen_edges[&edge_ref] + 1);
            } else {
                seen_edges.insert(edge_ref, 1);
            }
        }
    }
    let mut e = seen_edges.iter().collect::<Vec<(&EdgeIndex, &u32)>>();

    e.sort_by(|a, b| b.1.cmp(a.1));

    for (ei, _count) in e.iter().take(3) {
        graph.remove_edge(**ei);
    }

    let scc = tarjan_scc(&graph);
    println!("{:?}", scc[0].len() * scc[1].len());
}
