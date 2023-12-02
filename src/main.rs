// Use the day 1 function from day1.rs
mod day1;
mod day2;
fn main() {
    // take in the first argument as the day to run
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a day to run");
        return;
    }

    let day = args[1].parse::<u32>().unwrap();
    match day {
        1 => day1::day1(),
        2 => day2::day2(),
        _ => println!("Day {} not implemented yet", day),
    }
}
