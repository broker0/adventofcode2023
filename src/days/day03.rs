use std::collections::BTreeMap;

fn parse_input(input: &str) -> Vec<BTreeMap<usize, String>> {
    let mut result = Vec::new();

    for line in input.lines() {
        let mut number = String::new();
        let mut row = BTreeMap::new();

        for (pos, symbol) in line.chars().enumerate() {
            if symbol.is_digit(10) {
                number.push(symbol);
            } else {
                if number.len() > 0 {
                    row.insert(pos-number.len(), number.clone());
                    number.clear()
                }

                if symbol != '.' {
                    row.insert(pos, String::from(symbol));
                }
            }
        }

        // flush token
        if number.len() > 0 {
            row.insert(line.chars().count()-number.len(), number.clone());
            number.clear()
        }
        result.push(row);
    }

    result
}


fn find_numbers_around(x: usize, row: &BTreeMap<usize, String>, nums: &mut Vec<i32>) {
    let left = x.saturating_sub(5); // for five digit numbers
    let right = x.saturating_add(1);

    let marks = row.range(left..=right);

    for (num_left, mark) in marks {
        let num_right = num_left + mark.len();

        if *num_left <= x+1 && num_right >= x {
            if let Ok(num) = mark.parse::<i32>() {
                nums.push(num)
            }

        }
    }
}


fn find_symbols_around(x: usize, w: usize, row: &BTreeMap<usize, String>) -> i32 {
    let left = x.saturating_sub(1);
    let right = x.saturating_add(w);
    let mut result = 0;

    for (mark_pos, mark) in row.range(left..=right) {
        if let Ok(_) = mark.parse::<i32>() {
            continue    // skip numbers
        }

        if *mark_pos >= left && *mark_pos <= right {
            result += 1
        }

    }

    result
}


pub fn stage1_2(input: &String) -> String {
    // println!("{input}");
    let field = parse_input(input);

    let mut sum1 = 0;
    let mut sum2 = 0;

    for (row_num, row) in field.iter().enumerate() {
        for (pos, mark) in row.iter() {
            if let Ok(value) = mark.parse::<i32>() {
                let len = mark.len();
                // number
                let mut symbols = find_symbols_around(*pos, len, row);
                if row_num > 0 {
                    symbols += find_symbols_around(*pos, len, &field[row_num-1]);
                }
                if row_num + 1 < field.len() {
                    symbols += find_symbols_around(*pos, len, &field[row_num+1]);
                }

                sum1 += symbols *value;
            } else {
                assert_eq!(mark.len(), 1);
                match mark.as_str() {
                    "*" => {
                        // gear

                        let mut nums = Vec::new();
                        find_numbers_around(*pos, row, &mut nums);
                        if row_num > 0 {
                            find_numbers_around(*pos, &field[row_num-1], &mut nums);
                        };

                        if row_num + 1 < field.len() {
                            find_numbers_around(*pos, &field[row_num+1], &mut nums);
                        }

                        if nums.len() == 2 {
                            let ratio = nums.iter().fold(1, |acc, v| acc*v);
                            sum2 += ratio;
                        } else {
                            // not gear
                        }
                    }

                    "=" | "%" | "#" | "$" | "+" | "@" | "&" | "/" | "-" => {
                        // other symbols
                    }

                    _ => {
                        panic!("invalid symbol {mark}");
                    }
                }
            }
        }
    }

    format!("{sum1} {sum2}")
}

