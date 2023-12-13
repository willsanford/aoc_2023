use std::fs::read_to_string;

fn get_diffs(a: &String, b: &String) -> u32 {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() as u32
}
fn find_reflection(lines: &Vec<String>) -> Option<u32> {
    // Get all pairs of lines that are the same
    let pairs = (1..lines.len())
        .filter(|i| get_diffs(&lines[*i], &lines[*i - 1]) <= 1)
        .collect::<Vec<usize>>();

    pairs.iter().find_map(|i_| {
        let mut i = *i_;
        let mut j = i - 1;
        let mut diffs = 0;
        while i < lines.len() {
            diffs += get_diffs(&lines[i], &lines[j]);
            if j == 0 || diffs > 1 {
                break;
            }
            i += 1;
            j -= 1;
        }

        if diffs == 1 {
            return Some(*i_ as u32);
        }
        return None;
    })
}

fn solve_puzzle(puzzle: &str) -> u32 {
    let lines = puzzle
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let horizontal_ref = find_reflection(&lines);
    if horizontal_ref.is_some() {
        return horizontal_ref.unwrap() * 100;
    }

    // there is no horizontal reflection and we need to find the vertical reflection
    let v_lines: Vec<String> = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let vertical_ref = find_reflection(&v_lines);
    if vertical_ref.is_some() {
        return vertical_ref.unwrap();
    }
    unreachable!()
}

pub fn day13() {
    let filename = "data/day_13.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .into_iter()
        .map(solve_puzzle)
        .sum();

    println!("{:?}", ans);
}
