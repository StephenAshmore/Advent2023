use std::fs;
use regex::Regex;

pub fn word_matcher(word: &String) -> u32 {
    let num: u32;
    match word.as_str() {
        "one" => num = 1,
        "two" => num = 2,
        "three" => num = 3,
        "four" => num = 4,
        "five" => num = 5,
        "six" => num = 6,
        "seven" => num = 7,
        "eight" => num = 8,
        "nine" => num = 9,
        _ => num = 0,
    }
    return num;
}

pub fn last_char_check(chars: &Vec<char>) -> bool {
    // optimizing when we run the regex
    let last_char = chars[chars.len() - 1];
    let letter_bool: bool;
    match last_char {
        'e' => letter_bool = true,
        'o' => letter_bool = true,
        'r' => letter_bool = true,
        'x' => letter_bool = true,
        'n' => letter_bool = true,
        't' => letter_bool = true,
        _ => letter_bool = false,
    }
    return letter_bool;
}

pub fn check_for_number(chars: &Vec<char>) -> u32 {
    // check for character version of number
    let word: String = chars.iter().collect();
    let mut num: u32 = word_matcher(&word);
    
    // check for word at end of string
    // use last_char_check as an optimization
    if num == 0 && chars.len() > 3 && last_char_check(&chars) {
        let re = Regex::new(r"one$|two$|three$|four$|five$|six$|seven$|eight$|nine$").unwrap();
        let mat = re.find(&word);
        if mat.is_some() {
            num = word_matcher(&mat.unwrap().as_str().to_string());
        }
    }

    return num;
}

pub fn line_checker(line: &str) -> u32 {
    let mut first_number = 0;
    let mut last_number = 0;
    let mut first_number_found = false;
    let mut last_number_found = false;
    let mut chars = Vec::new();
    for c in line.chars() {
        if c.is_digit(10) {
            chars.clear();
            let number = c.to_digit(10).unwrap();
            if !first_number_found {
                first_number = number;
                first_number_found = true;
            } else {
                last_number = number;
                last_number_found = true;
            }
        } else {
            chars.push(c);
            let num = check_for_number(&chars);
            if num != 0 {
                if !first_number_found {
                    first_number = num;
                    first_number_found = true;
                } else {
                    last_number = num;
                    last_number_found = true;
                }
            }
        }
    }

    // put the digits together
    // make sure to handle if only first_number was found
    let mut final_number = 0;
    if first_number_found && last_number_found {
        final_number = first_number * 10 + last_number;
    } else if first_number_found {
        final_number = first_number * 10 + first_number;
    }
    println!("{}: {}", line, final_number);
    return final_number;
}


pub fn day1() {
    println!("Day 1, Loading File...");
    let contents = fs::read_to_string("./data/day1.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut sum = 0;

    for line in lines {
        sum += line_checker(&line);
    }
    println!("Sum: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::word_matcher;
    use super::line_checker;
    use super::check_for_number;
    use super::last_char_check;

    #[test]
    fn test_word_matcher() {
        assert_eq!(word_matcher(&"one".to_string()), 1);
        assert_eq!(word_matcher(&"two".to_string()), 2);
        assert_eq!(word_matcher(&"three".to_string()), 3);
        assert_eq!(word_matcher(&"four".to_string()), 4);
        assert_eq!(word_matcher(&"five".to_string()), 5);
        assert_eq!(word_matcher(&"six".to_string()), 6);
        assert_eq!(word_matcher(&"seven".to_string()), 7);
        assert_eq!(word_matcher(&"eight".to_string()), 8);
        assert_eq!(word_matcher(&"nine".to_string()), 9);
        assert_eq!(word_matcher(&"ten".to_string()), 0);
    }

    #[test]
    fn test_line_checker() {
        assert_eq!(line_checker(&"one".to_string()), 11);
        assert_eq!(line_checker(&"one two".to_string()), 12);
        assert_eq!(line_checker(&"one two three".to_string()), 13);
        assert_eq!(line_checker(&"one two three four".to_string()), 14);
        assert_eq!(line_checker(&"one two three four five".to_string()), 15);
        assert_eq!(line_checker(&"one two three four five six".to_string()), 16);
        assert_eq!(line_checker(&"one two three four five six seven".to_string()), 17);
        assert_eq!(line_checker(&"one two three four five six seven eight".to_string()), 18);
        assert_eq!(line_checker(&"one two three four five six seven eight nine".to_string()), 19);
    }

    #[test]
    fn test_line_checker_middle_number() {
        assert_eq!(line_checker(&"onethr2ee".to_string()), 12);
    }
    
    #[test]
    fn test_line_checker_example() {
        assert_eq!(line_checker(&"two1nine".to_string()), 29);
        assert_eq!(line_checker(&"eightwothree".to_string()), 83);
        assert_eq!(line_checker(&"abcone2threexyz".to_string()), 13);
        assert_eq!(line_checker(&"xtwone3four".to_string()), 24);
        assert_eq!(line_checker(&"4nineeightseven2".to_string()), 42);
        assert_eq!(line_checker(&"zoneight234".to_string()), 14);
        assert_eq!(line_checker(&"7pqrstsixteen".to_string()), 76);
    }

    #[test]
    fn test_line_checker_real() {
        assert_eq!(line_checker(&"cmfkrlslhgzprgkfive6seven".to_string()), 57);
        assert_eq!(line_checker(&"4threethree".to_string()), 43);
        assert_eq!(line_checker(&"threesix2fkzsjkr6six".to_string()), 36);
        assert_eq!(line_checker(&"8bcqmshjlnfivecrjtcsznrfive".to_string()), 85);
        assert_eq!(line_checker(&"11nine3six".to_string()), 16);
        assert_eq!(line_checker(&"326one1zvdzc".to_string()), 31);
        assert_eq!(line_checker(&"1sjttzbstpx6sgfzpgdltxseven15bvrbmccbzkbgdnkkhpd".to_string()), 15);
        assert_eq!(line_checker(&"9bbpksbnpdm".to_string()), 99);
        assert_eq!(line_checker(&"3rtcztcr".to_string()), 33);
        assert_eq!(line_checker(&"bk1sevenjbmncfiveninejp".to_string()), 19);
        assert_eq!(line_checker(&"seven62hjvttpk".to_string()), 72);
        assert_eq!(line_checker(&"rsnqnlgfgrxk8sfxhrlgmc1jnjgctclr7".to_string()), 87);
        assert_eq!(line_checker(&"3sbmxlshf5five4".to_string()), 34);
        assert_eq!(line_checker(&"8nprjs".to_string()), 88);
        assert_eq!(line_checker(&"three89".to_string()), 39);
        assert_eq!(line_checker(&"four399ljmdptjbgkthree".to_string()), 43);
        assert_eq!(line_checker(&"cthxllrzbseveneight7four".to_string()), 74);
        assert_eq!(line_checker(&"3gzvsfnxfive".to_string()), 35);
        assert_eq!(line_checker(&"seven".to_string()), 77);
        assert_eq!(line_checker(&"kqxncc74sjttqklx3lxpffbdlthreetwooneightnb".to_string()), 78);
    }

    #[test]
    fn test_last_char_check() {
        assert_eq!(last_char_check(&vec!['e']), true);
        assert_eq!(last_char_check(&vec!['o']), true);
        assert_eq!(last_char_check(&vec!['r']), true);
        assert_eq!(last_char_check(&vec!['x']), true);
        assert_eq!(last_char_check(&vec!['n']), true);
        assert_eq!(last_char_check(&vec!['t']), true);
        assert_eq!(last_char_check(&vec!['a']), false);
        assert_eq!(last_char_check(&vec!['b']), false);
        assert_eq!(last_char_check(&vec!['c']), false);
        assert_eq!(last_char_check(&vec!['f', '2', '3', 'f', 'a', 's', 'd', 'e']), true);
        assert_eq!(last_char_check(&vec!['s', 'd', 'f', 'h', '3', '4', '5', 'o']), true);
        assert_eq!(last_char_check(&vec!['c', 'v', 's', 'd', 'f', 'r']), true);
        assert_eq!(last_char_check(&vec!['q', 'w', 'e', 'r', 'x']), true);
        assert_eq!(last_char_check(&vec!['z', 'x', 'b', 'n', 'j', 'n']), true);
        assert_eq!(last_char_check(&vec!['q', 'w', 'e', 'r', 'a', 'a', 'a', 'a', 't']), true);
    }

    #[test]
    fn test_check_for_number_single_digit() {
        let chars = vec!['1'];
        let result = check_for_number(&chars);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_check_for_number_double_digit() {
        let chars = vec!['1', '2'];
        let result = check_for_number(&chars);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_check_for_number_word() {
        assert_eq!(check_for_number(&vec!['o', 'n', 'e']), 1);
        assert_eq!(check_for_number(&vec!['t', 'w', 'o']), 2);
        assert_eq!(check_for_number(&vec!['t', 'h', 'r', 'e', 'e']), 3);
        assert_eq!(check_for_number(&vec!['f', 'o', 'u', 'r']), 4);
        assert_eq!(check_for_number(&vec!['f', 'i', 'v', 'e']), 5);
        assert_eq!(check_for_number(&vec!['s', 'i', 'x']), 6);
        assert_eq!(check_for_number(&vec!['s', 'e', 'v', 'e', 'n']), 7);
        assert_eq!(check_for_number(&vec!['e', 'i', 'g', 'h', 't']), 8);
        assert_eq!(check_for_number(&vec!['n', 'i', 'n', 'e']), 9);
    }

    #[test]
    fn test_check_for_number_no_match() {
        let chars = vec!['a', 'b', 'c'];
        let result = check_for_number(&chars);
        assert_eq!(result, 0);
    }
}