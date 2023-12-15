use std::collections::HashMap;
use std::fs::read_to_string;

fn rotate_90_degrees(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();
    for i in 0..n {
        for j in i..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    for row in matrix {
        row.reverse();
    }
}

fn _print(data: &Vec<Vec<char>>) {
    for line in data {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
    println!();
}

fn tilt(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    data.iter()
        .map(|line_| {
            let mut line = line_.clone();
            let mut current: usize = line.len() - 1;
            'outer: loop {
                // Find the next open character.
                while line[current] != '.' {
                    if let Some(c) = current.checked_sub(1) {
                        current = c;
                    } else {
                        break 'outer;
                    }
                }
                let mut swap = current;
                while !(line[swap] == 'O' || line[swap] == '#') {
                    if let Some(c) = swap.checked_sub(1) {
                        swap = c;
                    } else {
                        break 'outer;
                    }
                }
                // If we have walked off the end of the line or found a bad stone, then do nothing, we are done.
                if line[swap] == '#' {
                    if let Some(c) = swap.checked_sub(1) {
                        current = c;
                    } else {
                        break 'outer;
                    }
                    continue;
                }
                line[swap] = '.';
                line[current] = 'O';
                if let Some(c) = current.checked_sub(1) {
                    current = c;
                } else {
                    break;
                }
            }
            line.clone()
        })
        .collect()
}

pub fn day14() {
    let filename = "data/day_14.txt";
    let binding = read_to_string(filename).unwrap();

    let mut data: Vec<Vec<char>> = binding
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();

    let mut cycles = 0;
    let mut states: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let cycle_start;
    loop {
        for _ in 0..4 {
            rotate_90_degrees(&mut data);
            data = tilt(&data);
        }
        cycles += 1;
        if let Some(c) = states.get(&data) {
            // Find the cycle length and where we are
            cycle_start = *c;
            break;
        }
        states.insert(data.clone(), cycles);
    }

    let remaining_cycles = 1000000000 - cycle_start;
    let cycle_size = cycles - cycle_start;
    let extra_cycles = remaining_cycles % cycle_size;

    // Do the extra cycles
    for _ in 0..extra_cycles {
        for _ in 0..4 {
            rotate_90_degrees(&mut data);
            data = tilt(&data);
        }
    }

    // Get the north support
    rotate_90_degrees(&mut data);
    let ans: u32 = data
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| *c == &'O')
                .map(|(i, _)| i as u32 + 1)
                .sum::<u32>()
        })
        .sum();

    println!("{:?}", ans);
}

pub fn _day14() {
    let filename = "data/day_14_ex.txt";
    let binding = read_to_string(filename).unwrap();

    let mut data: Vec<Vec<char>> = binding
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| line.chars().collect())
        .collect();

    // Transpose the data
    rotate_90_degrees(&mut data);
    data = tilt(&data);

    let ans: u32 = data
        .iter()
        .map(|line| {
            line.iter()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == &'O')
                .map(|(i, _)| i as u32 + 1)
                .sum::<u32>()
        })
        .sum();
    println!("{:?}", ans);
}
