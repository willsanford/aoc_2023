use std::{collections::HashMap, fs::read_to_string};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(vec: &[u64]) -> u64 {
    vec.iter().cloned().reduce(|a, b| lcm(a, b)).unwrap_or(1)
}

pub fn day8() {
    let filename = "data/day_8.txt";
    let bindings = read_to_string(filename).unwrap();

    let lines: Vec<&str> = bindings.split('\n').filter(|line| line.len() > 0).collect();
    let dirs: Vec<bool> = lines[0].chars().map(|c| c == 'L').collect();

    let m: HashMap<&str, (&str, &str)> = lines[1..]
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('=').collect();
            let first = &parts[0][0..3];
            let second = &parts[1][2..5];
            let third = &parts[1][7..10];

            (first, (second, third))
        })
        .collect();
    let starts: Vec<&str> = m
        .iter()
        .map(|(k, _v)| *k)
        .filter(|s| s.chars().nth(2).unwrap() == 'A')
        .collect();

    let cycle_lens: Vec<u64> = starts
        .iter()
        .map(|s| {
            let mut current: (&str, u64) = (s, 0);

            while !(current.0.chars().nth(2).unwrap() == 'Z') {
                current = dirs.iter().fold(current.clone(), |(c, a), d| {
                    if c.chars().nth(2).unwrap() == 'Z' {
                        return (c, a);
                    }
                    let new_place = {
                        if *d {
                            &m[&c].0
                        } else {
                            &m[&c].1
                        }
                    };

                    return (new_place, a + 1);
                });
            }
            return current.1;
        })
        .collect();

    println!("{:?}", lcm_of_vec(cycle_lens.as_slice()));
}

pub fn _day8() {
    let filename = "data/day_8.txt";
    let bindings = read_to_string(filename).unwrap();

    let lines: Vec<&str> = bindings.split('\n').filter(|line| line.len() > 0).collect();
    let dirs: Vec<bool> = lines[0].chars().map(|c| c == 'L').collect();

    let m: HashMap<String, (String, String)> = lines[1..]
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split('=').collect();
            let first = &parts[0][0..3];
            let second = &parts[1][2..5];
            let third = &parts[1][7..10];

            (first.to_string(), (second.to_string(), third.to_string()))
        })
        .collect();

    let mut current: (String, u64) = (String::from("AAA"), 0);

    while !current.0.eq(&String::from("ZZZ")) {
        current = dirs.iter().fold(current.clone(), |(c, a), d| {
            if c.eq(&String::from("ZZZ")) {
                return (c, a);
            }
            let new_place = {
                if *d {
                    &m[&c].0
                } else {
                    &m[&c].1
                }
            };

            return (new_place.clone(), a + 1);
        });
    }
    println!("{:?} - {:?}", current.0, current.1);
}
