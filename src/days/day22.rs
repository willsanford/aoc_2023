use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug)]
struct Rec {
    id: usize,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

// Assuming the same z, check if the two rectangles collide in x and y
fn check_collision(a: &Rec, b: &Rec) -> bool {
    if a.x.0 > b.x.1 || a.x.1 < b.x.0 {
        return false;
    }
    if a.y.0 > b.y.1 || a.y.1 < b.y.0 {
        return false;
    }
    true
}

pub fn day22() {
    let filename = "data/day_22.txt";

    let mut recs: Vec<Rec> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .into_iter()
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(i, line)| {
            let (l, r) = line.split_once('~').unwrap();
            let l_: Vec<i32> = l.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let r_: Vec<i32> = r.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            Rec {
                id: i,
                x: (l_[0], r_[0]),
                y: (l_[1], r_[1]),
                z: (l_[2], r_[2]),
            }
        })
        .collect();

    // [A, [B, C, D, ...]] A is supported by B, C, D, ...
    let mut support: HashMap<usize, Vec<usize>> = HashMap::new();
    // [A, [B, C, D, ...]] A supports B, C, D, ...
    let mut supported: HashMap<usize, Vec<usize>> = HashMap::new();

    // Sort the recs by the z
    recs.sort_by(|a, b| a.z.0.cmp(&b.z.0));

    for i in 0..recs.len() {
        let mut collision_z = 1; // Assume that the rec falls to the bottom
        let mut sup: Vec<usize> = Vec::new(); // Ids of blocks supporting at this z
        for j in (0..i).rev() {
            if check_collision(&recs[i], &recs[j]) {
                if recs[j].z.1 + 1 > collision_z {
                    collision_z = recs[j].z.1 + 1;
                    sup = vec![recs[j].id];
                } else if recs[j].z.1 + 1 == collision_z {
                    sup.push(recs[j].id);
                }
            }
        }
        support.insert(recs[i].id, sup);
        let h = recs[i].z.1 - recs[i].z.0;
        recs[i].z = (collision_z, collision_z + h);
    }

    // Invert the support map
    for (k, v) in support.iter() {
        for id in v {
            if supported.contains_key(id) {
                supported.get_mut(id).unwrap().push(*k);
            } else {
                supported.insert(*id, vec![*k]);
            }
        }
    }

    let mut ids: HashSet<usize> = support.keys().map(|k| *k).collect();
    let mut d: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in support.iter() {
        if v.len() == 1 {
            ids.remove(&v[0]);
            if d.contains_key(&v[0]) {
                d.get_mut(&v[0]).unwrap().push(*k);
            } else {
                d.insert(v[0], vec![*k]);
            }
        }
    }

    // Loop over all the individual ids and check the chain reaction
    let ans: u32 = recs
        .iter()
        .map(|r| {
            let id = r.id;
            let mut dis: HashSet<usize> = HashSet::new();
            dis.insert(id);
            let mut to_check: Vec<usize> = vec![id];
            while let Some(check) = to_check.pop() {
                if let Some(possibles) = supported.get(&check) {
                    let new_dis: Vec<usize> = possibles
                        .iter()
                        .filter(|temp| {
                            if let Some(temp_) = support.get(temp) {
                                temp_.iter().all(|d_| dis.contains(d_))
                            } else {
                                false
                            }
                        })
                        .map(|temp| *temp)
                        .collect();
                    dis.extend(new_dis.clone());
                    to_check.extend(new_dis.clone());
                }
            }

            (dis.len() - 1) as u32
        })
        .sum();

    println!("Part 1: {:?}", ids.len());
}
