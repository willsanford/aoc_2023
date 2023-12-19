use std::cmp;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Rule {
    rate: usize,
    cmp: cmp::Ordering,
    val: u64,
    skip: bool,
    next: String,
}

#[derive(Debug)]
struct Part {
    vals: [u64; 4],
}

pub fn day19() {
    let filename = "data/day_19.txt";

    let data: String = read_to_string(filename).unwrap();
    let (rules, parts) = data.split_once("\n\n").unwrap();

    let rules: HashMap<String, Vec<Rule>> = rules
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (ident, rest) = line.split_once('{').unwrap();
            let conds: Vec<&str> = rest.split(',').collect();
            let mut out: Vec<Rule> = conds[..conds.len() - 1]
                .iter()
                .map(|cond| {
                    let c = if cond.find('<').is_some() {
                        cmp::Ordering::Less
                    } else {
                        cmp::Ordering::Greater
                    };

                    let (rate, rest) = cond.split_once(&['<', '>']).unwrap();
                    let (val, next) = rest.split_once(':').unwrap();
                    let rate_e = match rate {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => panic!("Invalid rate"),
                    };
                    Rule {
                        rate: rate_e,
                        cmp: c,
                        val: val.parse::<u64>().unwrap(),
                        skip: false,
                        next: next.to_string(),
                    }
                })
                .collect();

            let last = conds[conds.len() - 1];
            let (next, _) = last.split_once('}').unwrap();

            out.push(Rule {
                rate: 0usize,
                cmp: cmp::Ordering::Equal,
                val: 0,
                skip: true,
                next: next.to_string(),
            });

            (ident.to_string(), out)
        })
        .collect();

    let mut accepted: Vec<[(u64, u64); 4]> = Vec::new();
    let mut rejected: Vec<[(u64, u64); 4]> = Vec::new();

    let mut to_check: Vec<([(u64, u64); 4], String)> = Vec::new();
    to_check.push(([(1, 4000); 4], "in".to_string()));

    while let Some(e) = to_check.pop() {
        let (mut vals, current) = e;
        let rules: &Vec<Rule> = rules.get(&current).unwrap();
        for r in rules {
            if r.skip {
                if r.next == "A" {
                    accepted.push(vals);
                } else if r.next == "R" {
                    rejected.push(vals);
                } else {
                    to_check.push((vals, r.next.clone()));
                }
                continue;
            }
            let old = vals[r.rate];
            let (t, f) = if r.cmp == cmp::Ordering::Less {
                ((old.0, r.val - 1), (r.val, old.1))
            } else {
                ((r.val + 1, old.1), (old.0, r.val))
            };

            let mut new_t = vals.clone();
            new_t[r.rate] = t;
            if r.next == "A" {
                accepted.push(new_t);
            } else if r.next == "R" {
                rejected.push(new_t);
            } else {
                to_check.push((new_t, r.next.clone()));
            }

            vals[r.rate] = f;
        }
    }
    let ans: u64 = accepted
        .iter()
        .map(|x| x.iter().map(|x_| (x_.1 - x_.0 + 1)).product::<u64>())
        .sum();
    println!("Accepted: {:?}", ans);
}

pub fn _day19() {
    let filename = "data/day_19.txt";

    let data: String = read_to_string(filename).unwrap();
    let (rules, parts) = data.split_once("\n\n").unwrap();

    let rules: HashMap<String, Vec<Rule>> = rules
        .split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (ident, rest) = line.split_once('{').unwrap();
            let conds: Vec<&str> = rest.split(',').collect();
            let mut out: Vec<Rule> = conds[..conds.len() - 1]
                .iter()
                .map(|cond| {
                    let c = if cond.find('<').is_some() {
                        cmp::Ordering::Less
                    } else {
                        cmp::Ordering::Greater
                    };

                    let (rate, rest) = cond.split_once(&['<', '>']).unwrap();
                    let (val, next) = rest.split_once(':').unwrap();
                    let rate_e = match rate {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => panic!("Invalid rate"),
                    };
                    Rule {
                        rate: rate_e,
                        cmp: c,
                        val: val.parse::<u64>().unwrap(),
                        skip: false,
                        next: next.to_string(),
                    }
                })
                .collect();

            let last = conds[conds.len() - 1];
            let (next, _) = last.split_once('}').unwrap();

            out.push(Rule {
                rate: 0usize,
                cmp: cmp::Ordering::Equal,
                val: 0,
                skip: true,
                next: next.to_string(),
            });

            (ident.to_string(), out)
        })
        .collect();

    let parts: Vec<Part> = parts
        .split('\n')
        .filter(|line| line.len() > 2)
        .map(|line| {
            let temp = line
                .chars()
                .filter(|c| c.is_digit(10) || *c == ',')
                .collect::<String>();
            let mut vals: [u64; 4] = [0; 4];
            for (i, s) in temp.split(',').enumerate() {
                vals[i] = s.parse::<u64>().unwrap();
            }
            Part { vals }
        })
        .collect();

    let ans: u64 = parts
        .iter()
        .filter(|part| {
            let mut current = "in".to_string();
            while !(current == "A" || current == "R") {
                let rules: &Vec<Rule> = rules.get(&current).unwrap();
                'inner: for r in rules {
                    if r.skip {
                        current = r.next.clone();
                        break 'inner;
                    }

                    if part.vals[r.rate].cmp(&r.val) == r.cmp {
                        current = r.next.clone();
                        break;
                    }
                }
            }
            current == "A"
        })
        .map(|part| part.vals.iter().sum::<u64>())
        .sum();
    println!("Part 1: {}", ans);
}
