// Use the day 1 function from day1.rs
mod days;

fn main() {
    // take in the first argument as the day to run
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a day to run");
        return;
    }

    let day = args[1].parse::<u32>().unwrap();
    match day {
        1 => days::day1::day1(),
        2 => days::day2::day2(),
        3 => days::day3::day3(),
        4 => days::day4::day4(),
        5 => days::day5::day5(),
        6 => days::day6::day6(),
        7 => days::day7::day7(),
        8 => days::day8::day8(),
        9 => days::day9::day9(),
        10 => days::day10::day10(),
        11 => days::day11::day11(),
        12 => days::day12::day12(),
        13 => days::day13::day13(),
        14 => days::day14::day14(),
        15 => days::day15::day15(),
        16 => days::day16::day16(),
        17 => days::day17::day17(),
        18 => days::day18::day18(),
        19 => days::day19::day19(),
        20 => days::day20::day20(),
        21 => days::day21::day21(),
        22 => days::day22::day22(),
        23 => days::day23::day23(),
        24 => days::day24::day24(),
        _ => println!("Day {} not implemented yet", day),
    }
}
