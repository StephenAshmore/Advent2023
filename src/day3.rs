use std::fs;

pub fn chain(schematic: &Vec<Vec<char>>, x: usize, y: usize, counted: &mut Vec<Vec<bool>>) -> u32 {
    // go left:
    let mut start = x;
    let mut cur_x = x;
    while cur_x > 0 {
        cur_x -= 1;
        let cell = schematic[y][cur_x];
        if cell.is_numeric() {
            // move the start position:
            start = cur_x;
        } else {
            break;
        }
    }

    // now put the number together:
    let mut str = String::new();
    while start < schematic[y].len() {
        let cell = schematic[y][start];
        if cell.is_numeric() {
            counted[y][start] = true;
            str.push(cell);
        } else {
            break;
        }
        start += 1;
    }
    return str.parse::<u32>().unwrap();
}

pub fn adjacent(schematic: &Vec<Vec<char>>, x: usize, y: usize, counted: &mut Vec<Vec<bool>>) -> u32{
    let mut sum = 0;
    for yoffset in [-1, 0, 1].iter() {
        for xoffset in [-1, 0, 1].iter() {
            if *xoffset == 0 && *yoffset == 0 {
                continue;
            }
            let x2 = x as i32 + xoffset;
            let y2 = y as i32 + yoffset;
            // guard against out of bounds:
            if x2 < 0 || y2 < 0 {
                continue;
            } else if x2 >= schematic[0].len() as i32 || y2 >= schematic.len() as i32 {
                continue;
            }
            let cell = schematic[y2 as usize][x2 as usize];
            
            // make sure cell is numeric and not already chained:
            if cell.is_numeric() && !counted[y2 as usize][x2 as usize] {
                // chain the numbers:
                sum += chain(&schematic, x2 as usize, y2 as usize, counted);
            }
        }
    }
    return sum;
}

pub fn day3_solver(schematic: &Vec<Vec<char>>, counted: &mut Vec<Vec<bool>>) -> u32 {
    let mut sum = 0;
    for (y, row) in schematic.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != '.' && !col.is_numeric() {
                // Assume it's a symbol:
                sum += adjacent(&schematic, x, y, counted);
            }
        }
    }
    return sum;
}

pub fn gear(schematic: &Vec<Vec<char>>, x: usize, y: usize, counted: &mut Vec<Vec<bool>>) -> u32 {
    let mut sum = 1;
    let mut found = 0;
    for yoffset in [-1, 0, 1].iter() {
        for xoffset in [-1, 0, 1].iter() {
            if *xoffset == 0 && *yoffset == 0 {
                continue;
            }
            let x2 = x as i32 + xoffset;
            let y2 = y as i32 + yoffset;
            // guard against out of bounds:
            if x2 < 0 || y2 < 0 {
                continue;
            } else if x2 >= schematic[0].len() as i32 || y2 >= schematic.len() as i32 {
                continue;
            }
            let cell = schematic[y2 as usize][x2 as usize];
            
            // make sure cell is numeric and not already chained:
            if cell.is_numeric() && !counted[y2 as usize][x2 as usize] {
                // chain the numbers:
                sum *= chain(&schematic, x2 as usize, y2 as usize, counted);
                found += 1;
            }
        }
    }
    if found == 2 {
        return sum;
    }
    return 0;
}

pub fn day3_solver_part2(schematic: &Vec<Vec<char>>, counted: &mut Vec<Vec<bool>>) -> u32 {
    let mut sum = 0;
    for (y, row) in schematic.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != '.' && *col == '*' {
                sum += gear(&schematic, x, y, counted);
            }
        }
    }
    return sum;
}

fn initialize_counted_vector(schematic: &Vec<Vec<char>>, counted: &mut Vec<Vec<bool>>) {
    for (y, row) in schematic.iter().enumerate() {
        counted.push(vec![]);
        for (_x, _col) in row.iter().enumerate() {
            counted[y].push(false);
        }
    }
}

pub fn day3() {
    println!("Day 3, Loading File...");
    let contents = fs::read_to_string("./data/day3.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();
    let schematic: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut counted: Vec<Vec<bool>> = vec![];
    initialize_counted_vector(&schematic, &mut counted);

    let final_sum = day3_solver(&schematic, &mut counted);
    println!("Day 3 Part 1 answer: {}", final_sum);

    let mut counted2: Vec<Vec<bool>> = vec![];
    initialize_counted_vector(&schematic, &mut counted2);
    let final_sum_2 = day3_solver_part2(&schematic, &mut counted2);
    println!("Day 3 Part 2 answer: {}", final_sum_2);
}

#[cfg(test)]
mod day3_tests {
    use super::day3_solver;
    use super::initialize_counted_vector;
    use super::chain;
    use super::day3_solver_part2;
    
    #[test]
    fn test_day3_solver_example() {
        let schematic: Vec<Vec<char>> = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ];
        let mut counted: Vec<Vec<bool>> = vec![];
        initialize_counted_vector(&schematic, &mut counted);
        let result = day3_solver(&schematic, &mut counted);
        let expected = 4361;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_day3_part2_solver_example() {
        let schematic: Vec<Vec<char>> = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ];
        let mut counted: Vec<Vec<bool>> = vec![];
        initialize_counted_vector(&schematic, &mut counted);
        let result = day3_solver_part2(&schematic, &mut counted);
        let expected = 467835;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_day3_solver_real_10() {
        let schematic: Vec<Vec<char>> = vec![
            vec!['.', '8', '5', '4', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '3', '6', '2', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '2', '7', '1', '.', '.', '.', '7', '3', '2', '.', '.', '.', '.', '.', '.', '.', '.', '8', '3', '8', '.', '.', '.', '.', '.', '.', '.', '.', '.', '2', '4', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '1', '1', '7', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '4', '5', '9', '.', '.', '.', '.', '.', '.', '.', '.', '7', '6', '7', '*', '6', '4', '8', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '$', '.', '.', '.', '&', '.', '.', '=', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '9', '7', '0', '.', '.', '.', '.', '.', '.', '.', '.', '.', '3', '6', '8', '.', '1', '2', '4', '.', '+', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '5', '7', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '6', '5', '3', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '7', '2', '3', '.', '.', '.', '.', '.', '3', '6', '6', '.', '.', '.', '.', '*', '4', '4', '3', '.', '.', '6', '0', '.', '.', '.', '.', '.', '.', '.', '.', '.', '5', '3', '6', '.', '.', '.', '.', '4', '4', '1', '.', '.', '.', '.', '4', '5', '.', '.', '8', '7', '9', '.', '.', '.', '.', '.', '7', '8', '9', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '7', '4', '9', '*', '.', '.', '.', '-', '.', '.', '.', '+', '.', '.', '3', '3', '0', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '2', '1', '5', '%', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '7', '2', '5', '.', '.', '.', '.', '.', '9', '5', '3', '.', '.', '.', '.', '.', '.', '.', '.', '%', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '*', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '6', '3', '9', '.', '.', '.', '.', '.', '.', '3', '3', '1', '.', '4', '1', '9', '.'],
        ];
        let mut counted: Vec<Vec<bool>> = vec![];
        initialize_counted_vector(&schematic, &mut counted);
        let result = day3_solver(&schematic, &mut counted);
        let expected = 10479;
        assert_eq!(expected, result);
    }

    #[test]
    fn test_chain() {
        let schematic = vec![
            vec!['1', '2', '3', 'a', '4', '0', '6'],
            vec!['7', '8', '9', 'b', '0', '1', '2'],
        ];
        let mut counted = vec![
            vec![false; 7],
            vec![false; 7],
        ];
        assert_eq!(chain(&schematic, 0, 0, &mut counted), 123);
        assert_eq!(chain(&schematic, 4, 0, &mut counted), 406);
        assert_eq!(chain(&schematic, 0, 1, &mut counted), 789);
        assert_eq!(chain(&schematic, 4, 1, &mut counted), 12);
    }
}