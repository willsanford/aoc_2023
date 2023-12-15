use std::fs::read_to_string;

pub fn day14() {
    let filename = "data/day_14.txt";

    let ans: u32 = read_to_string(filename)
        .unwrap()
        .split(",")
        .into_iter()
        .map(|_| 0)
        .sum();

    println!("{:?}", ans);
}
