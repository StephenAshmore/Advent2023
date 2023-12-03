use std::fs;

struct Limits {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn game_checker_2(line: &str) -> u32 {
    let mut split = line[5..].split(":");
    let _id = split.next().unwrap();
    let game_text = split.next().unwrap();
    let mut max_red = 1;
    let mut max_green = 1;
    let mut max_blue = 1;
    for run in game_text.split(";") {
        for example in run.split(",") {
            let mut csplit = example.trim().split(" ");
            let num = csplit.next().unwrap().parse::<u32>().unwrap();
            let color = csplit.next().unwrap();
            if color == "red" && num > max_red {
                max_red = num;
            } else if color == "green" && num > max_green {
                max_green = num;
            } else if color == "blue" && num > max_blue {
                max_blue = num;
            }
        }
    }
    return max_red * max_green * max_blue;
}

pub fn game_checker_1(line: &str, red: u32, green: u32, blue: u32) -> u32 {
    let mut split = line[5..].split(":");
    let id = split.next().unwrap();
    let game_text = split.next().unwrap();
    for run in game_text.split(";") {
        for example in run.split(",") {
            let mut csplit = example.trim().split(" ");
            let num = csplit.next().unwrap().parse::<u32>().unwrap();
            let color = csplit.next().unwrap();
            if color == "red" && num > red {
                return 0;
            } else if color == "green" && num > green {
                return 0;
            } else if color == "blue" && num > blue {
                return 0;
            }
        }
    }
    return id.parse::<u32>().unwrap();
}

pub fn day2() {
    println!("Day 2, Loading File...");
    let contents = fs::read_to_string("./data/day2.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    let limit = Limits {
        red: 12,
        green: 13,
        blue: 14
    };
    for line in lines {
        sum_part1 += game_checker_1(&line, limit.red, limit.green, limit.blue);
        sum_part2 += game_checker_2(&line);
    }
    println!("Day 2, Part 1 answer: {}", sum_part1);
    println!("Day 2, Part 2 answer: {}", sum_part2);
}

#[cfg(test)]
mod day2_tests {
    use super::game_checker_1;
    use super::game_checker_2;

    #[test]
    fn test_game_matcher_1() {
        assert_eq!(game_checker_1(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 12, 13, 14), 1)
    }

    #[test]
    fn test_game_matcher_2() {
        assert_eq!(game_checker_2(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(game_checker_2(&"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(game_checker_2(&"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(game_checker_2(&"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(game_checker_2(&"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
}