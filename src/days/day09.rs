fn difference(origin: &[i64]) -> (Vec<i64>, Vec<i64>) {
    let mut right = Vec::new();
    let mut left = Vec::new();

    let mut current = Vec::from(origin);
    right.push(*current.last().unwrap());
    left.push(*current.first().unwrap());

    while true {
        let mut new_current = Vec::new();

        for pair in current.windows(2) {
            new_current.push(pair[1]-pair[0])
        }

        current = new_current;

        right.push(*current.last().unwrap());
        left.push(*current.first().unwrap());

        if current.iter().all(|v| *v==0) {
            break
        }
    }

    (left, right)
}
pub fn run(input: &str) {
    let mut lines = input.lines();

    let mut sum_left = 0;
    let mut sum_right = 0;

    for line in lines {
        let series: Vec<_> = line.split_whitespace().into_iter().map(|v| v.parse::<i64>().unwrap()).collect();
        let (mut left, mut right) = difference(&series);
        left.reverse();
        right.reverse();

        let mut curr_right = 0;
        let mut curr_left = 0;

        for (l, r) in left.iter().skip(1).zip(right.iter().skip(1)) {
            curr_right = r + curr_right;
            curr_left = l - curr_left;
        }

        sum_left += curr_left;
        sum_right += curr_right;
    }

    println!("stage 1 and 2: {sum_left} {sum_right}")
}