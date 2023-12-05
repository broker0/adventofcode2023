use std::cmp::Ordering;
use std::str::Lines;
use std::time::Instant;

fn read_block(lines: &mut Lines) -> Vec<(u32, u32, u32)> {
    let mut result = Vec::new();

    if let Some(block_name) = lines.next() {
        println!("{block_name}");
    } else {
        return result   // end of file
    };

    loop {
        if let Some(line) = lines.next() {
            if line.len() > 0 {
                let ranges: Vec<u32> = line.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
                assert_eq!(ranges.len(), 3);
                // println!("{ranges:?}");
                result.push((ranges[0], ranges[1], ranges[2]));
            } else {
                break   // empty string, end of block
            }
        } else {
            break   // None, end of file
        }
    }

    result
}


pub fn run1(input: &String) {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let (seeds, seed_nums) = seeds.split_once(": ").unwrap();
    assert_eq!(seeds, "seeds");

    let mut seeds: Vec<u32> = seed_nums.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
    assert_eq!(lines.next().unwrap(), "");

    loop {
        let mut ranges = read_block(&mut lines);
        if ranges.len() == 0 {
            break; // no more blocks, eof
        }

        ranges.sort_by(|(dst_a, src_a, len_a), (dst_b, src_b, len_b)| {
            src_a.cmp(src_b)
        });

        for mut seed in seeds.iter_mut() {
            let range_pos = ranges.binary_search_by(|(dst,src,len)| -> Ordering {
                if *seed >= src+len {
                    Ordering::Less
                } else if *seed < *src {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }

            });

            if let Ok(v) = range_pos {
                let (dst, src, len) = ranges[v];
                *seed = (*seed - src) + dst;
            }

        }
    }

    seeds.sort();

    println!("final: {seeds:#?}")

}


pub fn run2(input: &String) {
    let t1 = Instant::now();

    let mut lines = input.lines();
    let seeds = lines.next().unwrap();
    let (seeds, seed_nums) = seeds.split_once(": ").unwrap();
    assert_eq!(seeds, "seeds");

    let mut seeds_ranges: Vec<u32> = seed_nums.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
    let mut seeds = Vec::with_capacity(3*1024*1024*1024);
    for chunk in seeds_ranges.chunks(2) {
        let start = chunk[0];
        let len = chunk[1];
        for seed in start..start+len {
            seeds.push(seed)
        }
    }

    println!("{}", seeds.len());

    assert_eq!(lines.next().unwrap(), "");

    loop {
        let mut ranges = read_block(&mut lines);
        if ranges.len() == 0 {
            break; // no more blocks, eof
        }

        ranges.sort_by(|(dst_a, src_a, len_a), (dst_b, src_b, len_b)| {
            src_a.cmp(src_b)
        });

        for mut seed in seeds.iter_mut() {
            let range_pos = ranges.binary_search_by(|(dst,src,len)| -> Ordering {
                if *seed >= src+len {
                    Ordering::Less
                } else if *seed < *src {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }

            });

            if let Ok(v) = range_pos {
                let (dst, src, len) = ranges[v];
                *seed = (*seed - src) + dst;
            }

        }
    }

    println!("final: {}", seeds.iter().min().unwrap());
    println!("{:?}", t1.elapsed());

}