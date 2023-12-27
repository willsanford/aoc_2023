use itertools::Itertools;
use prime_factorization::Factorization;
use std::collections::HashSet;
use std::fs::read_to_string;
use z3::ast::{Ast, Int, Real};
use z3::{Config, Context, Solver};
#[derive(Debug, Copy, Eq, PartialEq, Clone)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

fn part_1(p1: &Pos, p2: &Pos) -> bool {
    let min = 200000000000000i64;
    let max = 400000000000000i64;
    if p1.dx * p2.dy == p2.dx * p1.dy {
        return false;
    }

    let t1 = (p2.dy * (p1.x - p2.x) - p2.dx * (p1.y - p2.y)) / (p1.dy * p2.dx - p1.dx * p2.dy);
    let t2 = (p1.dy * (p2.x - p1.x) - p1.dx * (p2.y - p1.y)) / (p2.dy * p1.dx - p2.dx * p1.dy);
    if t1 < 0 || t2 < 0 {
        return false;
    }
    let ix = p1.x + t1 * p1.dx;
    let iy = p1.y + t1 * p1.dy;

    if min <= ix && ix <= max && min <= iy && iy <= max {
        return true;
    }

    false
}

pub fn day24() {
    let filename = "data/day_24.txt";

    let p: Vec<Pos> = read_to_string(filename)
        .unwrap()
        .split('\n')
        .filter(|line| line.len() > 0)
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once('@').unwrap();
            let vals_right = right
                .split(',')
                .filter(|s| s.len() > 0)
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let vals_left = left
                .split(',')
                .filter(|s| s.len() > 0)
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            Pos {
                x: vals_left[0],
                y: vals_left[1],
                z: vals_left[2],
                dx: vals_right[0],
                dy: vals_right[1],
                dz: vals_right[2],
            }
        })
        .collect();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Real::new_const(&ctx, "px");
    let py = Real::new_const(&ctx, "py");
    let pz = Real::new_const(&ctx, "pz");
    let vx = Real::new_const(&ctx, "vx");
    let vy = Real::new_const(&ctx, "vy");
    let vz = Real::new_const(&ctx, "vz");

    for hailstone in p.clone() {
        let pxn = Real::from_int(&Int::from_i64(&ctx, hailstone.x));
        let pyn = Real::from_int(&Int::from_i64(&ctx, hailstone.y));
        let pzn = Real::from_int(&Int::from_i64(&ctx, hailstone.z));
        let vxn = Real::from_int(&Int::from_i64(&ctx, hailstone.dx));
        let vyn = Real::from_int(&Int::from_i64(&ctx, hailstone.dy));
        let vzn = Real::from_int(&Int::from_i64(&ctx, hailstone.dz));
        let tn = Real::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_real().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_real().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_real().unwrap();
    println!("Part 2: {}", x.0 + y.0 + z.0);

    let ans = p
        .iter()
        .cartesian_product(p.clone())
        .filter(|(a, b)| **a != *b && part_1(a, b))
        .count();
    println!("Part 1: {:?}", ans / 2);
}
