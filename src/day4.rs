use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;

pub fn card_checker(card: &str) -> (u32, u32) {
    let game_text = card.split(':').nth(1).unwrap();
    let winning = game_text.split('|').next().unwrap().trim();
    let numbers = game_text.split('|').nth(1).unwrap().trim();

    // Set up the winning set:
    let mut winning_set: Vec<u32> = vec![];
    for (_i, w) in winning.split(' ').enumerate() {
        let num = w.trim().parse::<u32>().unwrap_or(u32::MAX);
        if num == u32::MAX {
            continue;
        }
        winning_set.push(num);
    }

    // now go through the numbers and check if they are in the winning set
    let mut sum: u32 = 0;
    let mut count: u32 = 0;
    for (_i, number) in numbers.split(' ').enumerate() {
        let num = number.parse::<u32>().unwrap_or_else(|_| 0);

        if winning_set.contains(&num) {
            let new_add = if sum == 0 { 1 } else { sum };

            sum += new_add;
            count += 1;
        }
    }

    return (sum, count);
}

fn get_card_id(card: &str) -> u32 {
    return card[4..].split(':').nth(0).unwrap().trim().parse::<u32>().unwrap_or(u32::MAX);
}

pub fn day4_part2_solver(cards: Vec<&str>) -> u64 {
    // add dynamic programming:
    let mut card_cache: HashMap<u32, (u32, u32)> = HashMap::new();

    // Keep a count of all cards that we've won/started with:
    let mut count: u64 = 0;

    // this ends up being a "recursive" solution,
    // as winning cards add copies of other cards that we need to check.
    // so we need to keep track of the cards we've already checked,
    // and not check them again. We'll do that by using a queue.
    let mut to_check: VecDeque<&str> = VecDeque::new();
    for card in &cards {
        count += 1;
        // Push the original card onto the queue:
        to_check.push_back(card);
    }

    // while there are cards in our to_check queue, check them:
    let mut iterations = 0;
    while to_check.len() > 0 {
        iterations += 1;
        if iterations % 10000 == 0 {
            println!("Total Iterations {} | Checking {} cards", iterations, to_check.len());
        }
        // pop a card off the front:
        let card = to_check.pop_front().unwrap();
        // get the id:
        let card_id = get_card_id(card);
        if card_id == u32::MAX {
            println!("Error: card_id is MAX card: {}", card);
            panic!("Error: card_id is MAX card: {}", card);
        }

        // Use part 1's card checker to see if this card is a winner
        let card_sum: u32;
        let wins: u32;
        if card_cache.contains_key(&card_id) {
            let (x, y) = card_cache.get(&card_id).unwrap();
            card_sum = *x;
            wins = *y;
        } else {
            (card_sum, wins) = card_checker(card);
            card_cache.insert(card_id, (card_sum, wins));
        }
        
        if card_sum > 0 {
            // handle the number of cards to add:
            // we will assume that the order of cards is equivalent to the ids
            // which is true for our puzzle input.
            let start_id = card_id + 1;
            let end_id = card_id + wins + 1;
            for i in start_id..end_id {
                // check if we've gone past the end of the OG cards list:
                if i > cards.len() as u32 {
                    break;
                }

                count += 1;
                if get_card_id(cards[(i-1) as usize]) != i as u32 {
                    panic!("Error: card id {} does not match expected id {}", get_card_id(cards[(i-1) as usize]), i);
                }
                // if it wins, we will add the next wins cards to the list of cards to check
                to_check.push_back(cards[(i-1) as usize]);
            }
        }
    }

    return count;
}

pub fn day4_part1_solver(cards: &Vec<&str>) -> u32 {
    let mut sum: u32 = 0;
    for card in cards {
        let (total, _wins) = card_checker(card);
        sum += total;
    }
    return sum;
}

pub fn day4() {
    println!("Day 4, Loading File...");
    let contents = fs::read_to_string("./data/day4.txt")
        .expect("Something went wrong reading the file");

    let cards: Vec<&str> = contents.lines().collect();

    println!("Day 4, Part 1: {}", day4_part1_solver(&cards));

    println!("Day 4, Part 2: {}", day4_part2_solver(cards));
}


#[cfg(test)]
mod day4_tests {
    use super::card_checker;
    use super::day4_part2_solver;
    use super::get_card_id;

    #[test]
    fn test_card_checker_example() {
        assert_eq!(card_checker(&"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), (8, 4));
        assert_eq!(card_checker(&"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), (2, 2));
        assert_eq!(card_checker(&"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), (2, 2));
        assert_eq!(card_checker(&"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), (1, 1));
        assert_eq!(card_checker(&"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), (0, 0));
        assert_eq!(card_checker(&"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), (0, 0));
    }

    #[test]
    fn test_card_weird_winning_numbers() {
        assert_eq!(card_checker(&"Card 6:  1 18  7 56 72 | 74 77 10 23 35  7 36 11"), (1, 1));
    }

    #[test]
    fn test_day4_part2_solver_example() {
        let cards: Vec<&str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let result = day4_part2_solver(cards);
        let expected = 30;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_card_id_function() {
        assert_eq!(get_card_id(&"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 1);
        assert_eq!(get_card_id(&"Card 77: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 77);
        assert_eq!(get_card_id(&"Card 100: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 100);
        assert_eq!(get_card_id(&"Card 999: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 999);
        assert_eq!(get_card_id(&"Card 1234: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 1234);
        assert_eq!(get_card_id(&"Card 5678: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 5678);
    }
}