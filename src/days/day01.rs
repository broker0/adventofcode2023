

pub fn stage1(input: &String) -> String {
    let mut total_sum = 0;

    for line in input.lines() {
        let first_digit = line.chars().find(|char| char.is_digit(10)).unwrap().to_digit(10).unwrap();
        let last_digit = line.chars().rev().find(|char| char.is_digit(10)).unwrap().to_digit(10).unwrap();

        // println!("{first_digit} {last_digit}");

        total_sum += first_digit * 10 + last_digit;
    }

    total_sum.to_string()
}


const DIGITS: [(&str, usize); 19] = [
    ("0", 0usize), ("1", 1), ("2", 2), ("3", 3), ("4", 4),
    ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
    ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
];


fn is_match_digit(line: &str, position: usize) -> Option<usize> {
    for (digit_str, digit_val) in &DIGITS {
        if position + digit_str.len() - 1 < line.len() {
            let sub_str = &line[position..position +digit_str.len()];
            if sub_str.eq(*digit_str) {
                return Some(*digit_val)
            }
        }
    }

    None
}

pub fn stage2(input: &String) -> String {
    let mut total_sum = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        for i in 0..line.len() {
            if first_digit.is_none() {
                first_digit = is_match_digit(line, i);
            }

            if last_digit.is_none() {
                last_digit = is_match_digit(line, line.len()-1-i)
            }

            if first_digit.is_some() && last_digit.is_some() {
                break
            }
        }

        let first_digit = first_digit.unwrap();
        let last_digit = last_digit.unwrap();

        // println!("{first_digit}, {last_digit} - {line}");

        total_sum += first_digit * 10 + last_digit;
    }

    total_sum.to_string()
}
