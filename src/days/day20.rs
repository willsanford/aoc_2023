use std::collections::HashMap;
use std::fs::read_to_string;

struct Mod_con {
    last: HashMap<String, bool>,
    next: Vec<String>,
}

struct Mod_ff {
    on: bool,
    next: Vec<String>,
}

enum Mod {
    Ff(Mod_ff),
    Con(Mod_con),
}

pub fn day20() {
    let filename = "data/day_20_ex.txt";

    let m: HashMap<String, Mod> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (name, next) = line.split_once(" -> ").unwrap();
            let next_s: Vec<String> = next.split(',').map(|s| s.to_string()).collect();
            match line.chars().nth(0).unwrap() {
                '&' => {}
                '%' => {}
                'b' => {
                    // Broadcast Node
                }
                _ => unreachable!(),
            }
        })
        .collect();

    println!("{:?}", map);
}
