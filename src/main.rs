use std::env;
mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    println!("Advent of Code day {}", day);

    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        _ => println!("Day {} not implemented yet", day),
    }
}
