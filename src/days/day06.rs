
fn find_ways(time: i64, dist: i64) -> i64 {
    let mut times = 0;
    for t in 1..time {
        let speed = time-t;
        if speed * t > dist {
            times += 1
        }
    }

    times
}

pub fn run1(input: &str) {

    let mut input = input.lines();
    let times: Vec<_> = input.next().unwrap()
        .split_whitespace()
        .collect();

    let dists: Vec<_> = input.next().unwrap()
        .split_whitespace()
        .collect();

    let mut ways = Vec::new();
    for (time, dist) in times[1..].iter().zip(dists[1..].iter()) {
        let time = time.parse::<i64>().unwrap();
        let dist = dist.parse::<i64>().unwrap();
        ways.push(find_ways(time, dist));

    }

    println!("{}", ways.into_iter().reduce(|a, b| a*b).unwrap());

    let time = times[1..].join("").parse::<i64>().unwrap();
    let dist = dists[1..].join("").parse::<i64>().unwrap();

    println!("{}", find_ways(time, dist));
}