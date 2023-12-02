use std::fs::File;
use std::io::{BufReader, Read};

mod days;

fn get_day_data(day: u8, test: bool) -> String {
    let file_name = format!("input/day{day:0>2}{}.txt", if test { "t" } else { "" } );

    println!("try read data from {file_name}");

    let f = File::open(&file_name).unwrap();
    let mut buf = BufReader::new(f);
    let mut result = String::new();
    let _result_size = buf.read_to_string(&mut result).unwrap();

    result
}



fn run_day01(test: bool) {
    let input = get_day_data(1, test);
    let stage1 = days::day01::stage1(&input);
    println!("result of stage 1:\n{stage1}");

    let stage2 = days::day01::stage2(&input);
    println!("result of stage 2:\n{stage2}");
}


fn run_day02(test: bool) {
    let input = get_day_data(2, test);

    let stage1 = days::day02::stage1(&input);
    println!("result of stage 1:\n{stage1}");

    let stage2 = days::day02::stage2(&input);
    println!("result of stage 2:\n{stage2}");
}

fn main() {
    let test = false;

    run_day01(test);
    run_day02(test);
}
