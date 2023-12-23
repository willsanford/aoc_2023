use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct ModCon {
    last: HashMap<String, bool>,
    next: Vec<String>,
}

#[derive(Debug, Clone)]
struct ModFF {
    on: bool,
    next: Vec<String>,
}

#[derive(Debug, Clone)]
struct ModBC {
    next: Vec<String>,
}

#[derive(Debug, Clone)]
enum Mod {
    Ff(ModFF),
    Con(ModCon),
    Bc(ModBC),
}

pub fn day20() {
    let filename = "data/day_20.txt";

    let mut m: HashMap<String, Mod> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (name, next) = line.split_once(" -> ").unwrap();
            let next_s: Vec<String> = next.split(',').map(|s| s.trim().to_string()).collect();
            let m = match line.chars().nth(0).unwrap() {
                '&' => Mod::Con(ModCon {
                    last: HashMap::new(),
                    next: next_s,
                }),
                '%' => Mod::Ff(ModFF {
                    on: false,
                    next: next_s,
                }),
                'b' => Mod::Bc(ModBC { next: next_s }),
                _ => unreachable!(),
            };
            // remove the first character if its not the broadcaster
            if line.chars().nth(0).unwrap() != 'b' {
                (name[1..].to_string(), m)
            } else {
                (name.to_string(), m)
            }
        })
        .collect();

    // Get the name of each con mod
    let mut con_names: HashMap<String, Vec<String>> = m
        .clone()
        .iter()
        .filter(|(_, md)| match md {
            Mod::Con(_) => true,
            _ => false,
        })
        .map(|(name, _)| (name.clone(), Vec::new()))
        .collect();

    // For each other mod that leads to a con mod, add it to the con mod's last
    m.iter_mut().for_each(|(name, md)| {
        if let Mod::Con(_con) = md {
        } else if let Mod::Ff(md_) = md {
            md_.next.iter().for_each(|next| {
                if con_names.contains_key(next) {
                    con_names.get_mut(next).unwrap().push(name.clone());
                }
            });
        } else if let Mod::Bc(md_) = md {
            md_.next.iter().for_each(|next| {
                if con_names.contains_key(next) {
                    con_names.get_mut(next).unwrap().push(name.clone());
                }
            });
        }
    });

    // For each con mod, add the last mods to the con mod's last
    con_names.iter_mut().for_each(|(name, last)| {
        if let Mod::Con(con) = m.get_mut(name).unwrap() {
            con.last = last
                .iter()
                .map(|name| (name.clone(), false))
                .collect::<HashMap<String, bool>>();
        }
    });

    let mut pulses: Vec<(String, String, bool)> = Vec::new(); // from, to, on

    // These are the four target mods that need to be hit
    let mut targets = vec!["hh", "lk", "fn", "fh"];
    let mut cycles: Vec<u64> = Vec::new();
    let mut iters: u64 = 0;
    while cycles.len() < 4 {
        iters += 1;
        pulses.push(("NULL".to_string(), "broadcaster".to_string(), false));
        while let Some((from, to, on)) = pulses.pop() {
            if let Some(t) = targets.iter().position(|&t| t == to) {
                if !on {
                    targets.remove(t);
                    cycles.push(iters);
                    println!("Got high on {} at {}", to, iters);
                }
            }

            match m.get_mut(&to) {
                Some(Mod::Bc(bc)) => {
                    bc.next.iter().for_each(|next| {
                        pulses.push((to.clone(), next.clone(), on));
                    });
                }
                Some(Mod::Con(con)) => {
                    con.last.insert(from.clone(), on);
                    let p = !con.last.values().all(|&v| v);
                    con.next.iter().for_each(|next| {
                        pulses.push((to.clone(), next.clone(), p));
                    });
                }
                Some(Mod::Ff(ff)) => {
                    if !on {
                        ff.on = !ff.on;
                        ff.next.iter().for_each(|next| {
                            pulses.push((to.clone(), next.clone(), ff.on));
                        });
                    }
                }
                None => {}
            }
        }
    }
    println!("Part 2: {}", cycles.iter().product::<u64>());
}
