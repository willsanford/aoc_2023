use std::fs::read_to_string;

pub fn day6() {
    let filename = "data/day_6.txt";
    let binding = read_to_string(filename).unwrap();
    let arrs = binding
        .split('\n')
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|line| {
            if line.len() < 2 {
                return Err("Invalid input");
            }
            Ok(line[1]
                .trim()
                .chars()
                .filter(|x| !x.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
                .unwrap())
        })
        .filter_map(Result::ok)
        .collect::<Vec<u64>>();

    let time = arrs[0];
    let dist = arrs[1];
    let mut ans: u64 = 0;

    for i in 1..time {
        if i * (time - i) > dist {
            ans = time - (2 * i) + 1;
            break;
        }
    }

    println!("{:?}", ans);
}
pub fn _day6() {
    let filename = "data/day_6.txt";
    let binding = read_to_string(filename).unwrap();
    let arrs = binding
        .split('\n')
        .map(|line| line.split(':').collect::<Vec<&str>>())
        .map(|line| {
            if line.len() < 2 {
                return Err("Invalid input");
            }
            Ok(line[1]
                .trim()
                .split(' ')
                .map(|x| x.trim().parse::<u32>())
                .filter_map(Result::ok)
                .collect::<Vec<u32>>())
        })
        .filter_map(Result::ok)
        .collect::<Vec<Vec<u32>>>();

    let out: u32 = arrs[0]
        .iter()
        .zip(arrs[1].iter())
        .map(|(x, y)| {
            let mut ans: u32 = 0;
            for i in 1..*x {
                if i * (x - i) > *y {
                    ans = ans + 1;
                }
            }
            ans
        })
        .product();

    println!("{:?}", out);
}
