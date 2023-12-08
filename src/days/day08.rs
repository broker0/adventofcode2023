use std::collections::{BTreeMap, BTreeSet};


fn factors(n: i64) -> Vec<i64> {
    let mut result = Vec::new();
    let mut num = n;
    let mut divisor = 2;

    while num > 1 {
        while num % divisor == 0 {
            result.push(divisor);
            num /= divisor;
        }
        divisor += 1;
    }

    result
}

pub fn run(input: &str) {
    let mut lines = input.lines();

    let instruction = lines.next().unwrap();
    lines.next().unwrap();
    let mut nodes = BTreeMap::<&str, (&str, &str)>::new();

    for line in lines {
        let (src, dests) = line.split_once(" = ").unwrap();
        let (l_dest, r_dest) = dests.split_once(", ").unwrap();
        let l_dest = l_dest.trim_matches('(');
        let r_dest = r_dest.trim_matches(')');

        nodes.insert(src, (&l_dest, r_dest));
    }

    // part 1
    let mut current = "AAA";
    let mut step_counter = 0;

    for step in instruction.chars().cycle() {
        let node = nodes.get(&current).unwrap();
        match step {
            'L' => current = node.0,
            'R' => current = node.1,

            _ => panic!("Invalid direction"),
        }
        step_counter += 1;
        if current == "ZZZ" {
            break
        }
    }

    println!("Stage 1: {step_counter}");

    // part 2
    let mut currents: Vec<_> = nodes.keys().filter_map(|k| {
        if k.chars().last().unwrap() == 'A' {
            Some(*k)
        } else {
            None
        }
    }).collect();

    let mut step_counters = Vec::new();

    for (idx, current) in currents.iter().enumerate() {
        let mut step_counter: i64 = 0;
        let mut curr = *current;

        for step in instruction.chars().cycle() {
            let node = nodes.get(curr).unwrap();
            curr = match step {
                'L' => node.0,
                'R' => node.1,

                _ => panic!("Invalid direction"),
            };

            step_counter += 1;

            if curr.chars().last().unwrap() == 'Z' {
                break
            }
        }

        step_counters.push(step_counter);   // save steps for current start position
        println!("{idx} {current } {step_counter}");
    }

    let mut primes = BTreeSet::new();
    step_counters.iter().for_each(|n| {
        primes.extend(factors(*n));
    });

    println!("{primes:?}");

    let total_steps: i64 = primes.iter().product();
    println!("Stage 2: {total_steps}");
}