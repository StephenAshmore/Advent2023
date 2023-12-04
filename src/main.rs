use std::env;
mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    println!("Advent of Code day {}", day);

    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        "3" => day3::day3(),
        _ => println!("Day {} not implemented yet", day),
    }
}
