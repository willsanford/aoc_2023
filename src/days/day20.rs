//use std::collections::HashMap;
//
//use std::fs::read_to_string;
/*
#[derive(Debug, Clone)]
struct Mod_con {
    last: HashMap<String, bool>,
    next: Vec<String>,
}

#[derive(Debug, Clone)]
struct Mod_ff {
    on: bool,
    next: Vec<String>,
}

#[derive(Debug, Clone)]
struct Mod_bc {
    next: Vec<String>,
}

#[derive(Debug, Clone)]
enum Mod {
    Ff(Mod_ff),
    Con(Mod_con),
    Bc(Mod_bc),
}
*/

pub fn day20() {}
/*
pub fn day20() {
    let filename = "data/day_20_ex.txt";

    let mut m: HashMap<String, Mod> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (name, next) = line.split_once(" -> ").unwrap();
            let next_s: Vec<String> = next.split(',').map(|s| s.to_string()).collect();
            let m = match line.chars().nth(0).unwrap() {
                '&' => Mod::Con(Mod_con {
                    last: HashMap::new(),
                    next: next_s,
                }),
                '%' => Mod::Ff(Mod_ff {
                    on: false,
                    next: next_s,
                }),
                'b' => Mod::Bc(Mod_bc { next: next_s }),
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
        if let Mod::Con(con) = md {
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




}
*/
